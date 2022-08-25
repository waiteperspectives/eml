import itertools
from dataclasses import dataclass
from typing import List, Literal, Union
from uuid import uuid4

from svg import SVG, Line, M, Path, Q, Rect, Text, Translate, TSpan

NODE_WIDTH = 300
NODE_HEIGHT = 150
PAD = 150


def line_intersection(line1, line2):
    xdiff = (line1[0][0] - line1[1][0], line2[0][0] - line2[1][0])
    ydiff = (line1[0][1] - line1[1][1], line2[0][1] - line2[1][1])

    def det(a, b):
        return a[0] * b[1] - a[1] * b[0]

    div = det(xdiff, ydiff)
    if div == 0:
        raise Exception("lines do not intersect")

    d = (det(*line1), det(*line2))
    x = det(d, xdiff) / div
    y = det(d, ydiff) / div
    return x, y


@dataclass
class Point:
    x: int = 0
    y: int = 0
    id: str = uuid4().hex


@dataclass
class Node:
    type: Literal["event", "command", "job", "form", "view"]
    text: str = ""
    origin: Point = Point()
    width: int = NODE_WIDTH
    height: int = NODE_HEIGHT
    id: str = uuid4().hex

    def fill_color(self):
        match self.type:
            case "event":
                return "#f7a660"
            case "command":
                return "#60b3f7"
            case "job":
                return "#ffffff"
            case "form":
                return "#ffffff"
            case "view":
                return "#60f765"
            case _:
                raise NotImplementedError()

    def top_anchor(self) -> Point:
        x = int(self.origin.x + self.width / 2)
        y = self.origin.y
        return Point(x=x, y=y)

    def right_anchor(self) -> Point:
        x = self.origin.x + self.width
        y = int(self.origin.y + self.height / 2)
        return Point(x=x, y=y)

    def bottom_anchor(self) -> Point:
        x = int(self.origin.x + self.width / 2)
        y = int(self.origin.y + self.height)
        return Point(x=x, y=y)

    def left_anchor(self) -> Point:
        x = self.origin.x
        y = int(self.origin.y + self.height / 2)
        return Point(x=x, y=y)

    def anchors(self) -> List[Point]:
        return [
            self.top_anchor(),
            self.right_anchor(),
            self.bottom_anchor(),
            self.left_anchor(),
        ]

    def render(self):
        return [
            Rect(
                x=self.origin.x,
                y=self.origin.y,
                width=self.width,
                height=self.height,
                stroke="black",
                fill=self.fill_color(),
                stroke_width=2,
            ),
            Text(
                transform=[
                    Translate(x=self.origin.x + PAD / 4, y=self.origin.y + PAD / 6)
                ],
                x=0,
                y=0,
                elements=[
                    TSpan(
                        text=self.id,
                        x=0,
                        dy="1rem",  # type: ignore
                        font_weight="bold",
                        font_size="larger",  # type: ignore
                    ),
                    TSpan(
                        text="=" * len(self.id),
                        x=0,
                        dy="1rem",  # type: ignore
                    ),
                ]
                + [
                    TSpan(
                        text=line,
                        x=0,
                        dy="1rem",  # type: ignore
                    )
                    for line in self.text.split("\n")
                ],
            ),
        ]


@dataclass
class Arrow:
    begin_at: Node
    end_at: Node
    id: str = uuid4().hex

    def get_points(self):
        lookup = {
            ("job", "command"): ("right_anchor", "top_anchor"),
            ("form", "command"): ("right_anchor", "top_anchor"),
            ("command", "event"): ("bottom_anchor", "left_anchor"),
            ("event", "view"): ("right_anchor", "bottom_anchor"),
            ("view", "job"): ("top_anchor", "left_anchor"),
            ("view", "form"): ("top_anchor", "left_anchor"),
        }
        key = (self.begin_at.type, self.end_at.type)
        begin_meth, end_meth = lookup[key]  # type: ignore
        points = (
            getattr(self.begin_at, begin_meth)(),
            getattr(self.end_at, end_meth)(),
        )
        return points

    def render(self):
        left, right = self.get_points()
        from_job_or_form = self.begin_at.type in ["job", "form"]
        from_cmd = self.begin_at.type == "command"
        from_evt = self.begin_at.type == "event"
        from_view = self.begin_at.type == "view"
        if from_job_or_form:
            vertical = ((right.x, right.y), (right.x, 99999))
            horizontal = ((left.x, left.y), (0, left.y))
            control_point = line_intersection(vertical, horizontal)
        elif from_cmd:
            vertical = ((left.x, left.y), (left.x, 99999))
            horizontal = ((right.x, right.y), (0, right.y))
            control_point = line_intersection(vertical, horizontal)
        elif from_evt:
            vertical = ((right.x, right.y), (right.x, 99999))
            horizontal = ((left.x, left.y), (0, left.y))
            control_point = line_intersection(vertical, horizontal)
        elif from_view:
            vertical = ((left.x, left.y), (left.x, 99999))
            horizontal = ((right.x, right.y), (0, right.y))
            control_point = line_intersection(vertical, horizontal)
        else:
            raise NotImplementedError()
        return [
            Path(
                d=[
                    M(left.x, left.y),
                    Q(control_point[0], control_point[1], right.x, right.y),
                ],
                fill="none",
                stroke="black",
                stroke_width=2,
            )
        ]


@dataclass
class Swimlane:
    top: int = 0
    bottom: int = 0
    width: int = 0

    def swimlane_top(self) -> Line:
        return Line(
            x1=0,
            x2=(0 + self.width),
            y1=self.top,
            y2=self.top,
            stroke_width=3,
            stroke="black",
        )

    def swimlane_bottom(self) -> Line:
        return Line(
            x1=0,
            x2=(0 + self.width),
            y1=self.bottom,
            y2=self.bottom,
            stroke_width=3,
            stroke="black",
        )

    def render(self) -> List[Line]:
        return [
            self.swimlane_top(),
            self.swimlane_bottom(),
        ]


Element = Union[Node, Arrow, Swimlane]


class Model:
    def __init__(self):
        self.width = 1000
        self.height = 1000
        self.nodes = []
        self.arrows = []
        self.swimlane = None

    def add_node(self, node):
        self.nodes.append(node)
        return self

    def add_arrow(self, arrow):
        self.arrows.append(arrow)
        return self

    def node(self, node_id):
        return [x for x in self.nodes if x.id == node_id][0]

    def add_swimlane(self):
        self.swimlane = Swimlane(
            top=int(self.height / 3), bottom=int(self.height / 3 * 2), width=self.width
        )

    def set_dimensions(self):
        self.height = NODE_HEIGHT * 3 + PAD * 6
        self.width = PAD + sum([PAD, NODE_WIDTH]) * len(self.nodes) + PAD
        self.add_swimlane()
        x_pos = PAD
        for node in self.nodes:
            match node.type:
                case "job":
                    node.origin = Point(x=x_pos, y=PAD)
                    x_pos = x_pos + node.width + PAD
                case "form":
                    node.origin = Point(x=x_pos, y=PAD)
                    x_pos = x_pos + node.width + PAD
                case "command":
                    node.origin = Point(x=x_pos, y=PAD + node.height + PAD + PAD)
                    x_pos = x_pos + node.width + PAD
                case "event":
                    node.origin = Point(
                        x=x_pos,
                        y=PAD + node.height + PAD + PAD + node.height + PAD + PAD,
                    )
                    x_pos = x_pos + node.width + PAD
                case "view":
                    node.origin = Point(x=x_pos, y=PAD + node.height + PAD + PAD)
                    x_pos = x_pos + node.width + PAD
                case _:
                    raise NotImplementedError()

    def render(self):
        swimlane_els = self.swimlane.render()  # type: ignore
        node_els = list(itertools.chain([node.render() for node in self.nodes]))
        arrow_els = list(itertools.chain([arrow.render() for arrow in self.arrows]))
        return SVG(
            width=self.width,
            height=self.height,
            elements=swimlane_els + node_els + arrow_els,  # type: ignore
        )

    @classmethod
    def from_yaml(cls, parsed_yml):
        model = cls()
        for entry in parsed_yml:
            obj_type = list(entry.keys())[0]
            rec = entry[obj_type]
            match obj_type.lower():
                case "form" | "job" | "command" | "event" | "view":
                    model.add_node(
                        Node(
                            id=rec["id"],
                            type=obj_type.lower(),
                            text=rec.get("text", ""),
                        )
                    )
                case "arrow" | "=>":
                    model.add_arrow(
                        Arrow(
                            begin_at=model.node(rec["begin_at"]),
                            end_at=model.node(rec["end_at"]),
                        )
                    )
                case _:
                    raise NotImplementedError()
        model.set_dimensions()
        return model
