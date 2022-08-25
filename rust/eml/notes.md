When deserializing yaml into the domain types, you see where custom parser and
serializer might be valuable. `yaml_rust` effectively parses the yaml string,
but returns unvalidated generic data structures like vecs and hashmaps. Then
during the ingestion process where you convert into the domain model, you have
to validate the shape of the generics anyways.
