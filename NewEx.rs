#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataType;

    #[test]
    fn smoke() {
        let mut scope = Scope::new_root_scope();
        let mut graph = scope.graph_mut();
        let mut c = graph.new_operation("Const", "Const").unwrap();
        c.set_attr_tensor("value", 3.0f32.into()).unwrap();
        c.set_attr_type("dtype", DataType::Float).unwrap();
        c.finish().unwrap();
    }

    #[test]
    fn uniquification() {
        let scope = Scope::new_root_scope();
        assert_eq!(&scope.new_sub_scope("foo").name, "foo");
        assert_eq!(&scope.new_sub_scope("foo").name, "foo_1");
        let foo_1 = scope.new_sub_scope("foo");
        assert_eq!(&foo_1.name, "foo_2");
        assert_eq!(&foo_1.new_sub_scope("bar").name, "foo_2/bar");
        assert_eq!(&foo_1.new_sub_scope("bar").name, "foo_2/bar_1");
        assert_eq!(&foo_1.new_sub_scope("bar").name, "foo_2/bar_2");
    }

    #[test]
    fn get_unique_name_for_op() {
        let scope = Scope::new_root_scope();
        assert_eq!(scope.get_unique_name_for_op("Add"), "Add");
        assert_eq!(scope.get_unique_name_for_op("Add"), "Add_1");
        let foo = scope.new_sub_scope("foo");
        assert_eq!(foo.get_unique_name_for_op("Add"), "foo/Add");
        assert_eq!(foo.get_unique_name_for_op("Add"), "foo/Add_1");
        let bar = foo.with_op_name("bar");
        assert_eq!(bar.get_unique_name_for_op("Add"), "foo/bar");
        assert_eq!(bar.get_unique_name_for_op("Add"), "foo/bar_1");
    }
}