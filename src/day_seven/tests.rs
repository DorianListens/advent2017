mod part_one {
    use day_seven::*;
    const TEST_INPUT: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn it_can_parse_a_leaf() {
        let tower = parse_input("pbga (66)");

        assert_eq!(tower.nodes.len(), 1);
        match *tower.first() {
            Node::Leaf(ref name, _) => assert_eq!(name, "pbga"),
            Node::Node(_, _, _) => assert!(false, "wrong node type"),
        }
    }

    #[test]
    fn it_can_parse_a_node() {
        let tower = parse_input("ugml (68) -> gyxo, ebii, jptl");

        assert_eq!(tower.nodes.len(), 1);
        match *tower.first() {
            Node::Leaf(_, _) => assert!(false, "wrong node type"),
            Node::Node(ref name, _, ref edges) => {
                assert_eq!(name, "ugml");
                assert_eq!(edges.len(), 3);
                assert_eq!(edges[0], "gyxo");
                assert_eq!(edges[2], "jptl");
            }
        }
    }

    #[test]
    fn it_can_parse_the_test_input() {
        let tower = parse_input(TEST_INPUT);

        assert_eq!(tower.nodes.len(), 13);
        match tower.nodes[5] {
            Node::Leaf(_, _) => assert!(false, "wrong node type"),
            Node::Node(ref name, _, ref edges) => {
                assert_eq!(name, "fwft");
                assert_eq!(edges.len(), 3);
                assert_eq!(edges[0], "ktlj");
                assert_eq!(edges[2], "xhth");
            }
        }
    }

    #[test]
    fn it_finds_the_root_node() {
        let tower = parse_input(TEST_INPUT);

        match *tower.root() {
            Node::Leaf(_, _) => assert!(false, "wrong node type"),
            Node::Node(ref name, _, ref edges) => {
                assert_eq!(name, "tknk");
                assert_eq!(edges.len(), 3);
                assert_eq!(edges[0], "ugml");
            }
        }
    }

    #[test]
    fn the_answer_is() {
        let tower = parse_input(input::INPUT);

        match *tower.root() {
            Node::Leaf(_, _) => assert!(false, "wrong node type"),
            Node::Node(ref name, _, _) => {
                assert_eq!(name, "gynfwly");
            }
        }
    }
}
