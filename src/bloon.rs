use std::collections::HashMap;

macro_rules! bloon_graph {
    ($($name: expr => ($hp: expr; $($prop: expr),*; $($child: expr $(=> ($($cprop: expr),*))?),*),)*) => {{
        let mut map = HashMap::new();
        $(
            map.insert($name, BloonNode {
                hp: $hp,
                properties: {
                    #[allow(unused_mut)]
                    let mut props = BloonProperty::empty();
                    $(
                        props |= $prop;
                    )*
                    props
                },
                children: vec![$(
                    (
                        $child,
                        {
                            #[allow(unused_mut)]
                            let mut props = BloonProperty::empty();
                            $($(
                                props |= $cprop;
                            )*)?
                            props
                        }
                    )
                ),*]
            });
        )*
        map
    }};
}

bitflags::bitflags! {
    pub struct BloonProperty: u8 {
        /// Can't be seen.
        const CAMO               = 0b00000001;
        /// Regenerates back over time.
        const REGEN              = 0b00000100;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BloonName {
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
    Black,
    White,
    Purple,
    Zebra,
    Rainbow,
    Ceramic,
    MOAB,
    BFB,
    ZOMG,
    BAD,
    DDT,
    Lead,
}

#[derive(Debug, Clone)]
pub struct BloonNode {
    pub hp: u32,
    pub properties: BloonProperty,
    children: Vec<(BloonName, BloonProperty)>,
}

pub struct BloonGraph {
    pub nodes: HashMap<BloonName, BloonNode>,
}

impl BloonGraph {
    pub fn new() -> Self {
        // BTD 6 Graph.
        let nodes = bloon_graph!(
            BloonName::Red => (1;; ),
            BloonName::Blue => (1;; BloonName::Red),
            BloonName::Green => (1;; BloonName::Blue),
            BloonName::Yellow => (1;; BloonName::Green),
            BloonName::Pink => (1;; BloonName::Yellow),

            BloonName::Purple => (1;; BloonName::Pink, BloonName::Pink),
            BloonName::Black => (1;; BloonName::Pink, BloonName::Pink),
            BloonName::White => (1;; BloonName::Pink, BloonName::Pink),

            BloonName::Lead => (1;; BloonName::Black, BloonName::Black),
            BloonName::Zebra => (1;; BloonName::Black, BloonName::White),

            BloonName::Rainbow => (1;; BloonName::Zebra, BloonName::Zebra),
            BloonName::Ceramic => (10;; BloonName::Rainbow, BloonName::Rainbow),

            BloonName::MOAB => (200;; BloonName::Ceramic, BloonName::Ceramic, BloonName::Ceramic, BloonName::Ceramic),
            BloonName::BFB => (700;; BloonName::MOAB, BloonName::MOAB, BloonName::MOAB, BloonName::MOAB),
            BloonName::ZOMG => (4000;; BloonName::BFB, BloonName::BFB, BloonName::BFB, BloonName::BFB),
            BloonName::BAD => (20_000;; BloonName::ZOMG, BloonName::ZOMG, BloonName::DDT, BloonName::DDT, BloonName::DDT),
            BloonName::DDT => (400;
                BloonProperty::CAMO;
                BloonName::Ceramic => (BloonProperty::CAMO, BloonProperty::REGEN),
                BloonName::Ceramic => (BloonProperty::CAMO, BloonProperty::REGEN),
                BloonName::Ceramic => (BloonProperty::CAMO, BloonProperty::REGEN),
                BloonName::Ceramic => (BloonProperty::CAMO, BloonProperty::REGEN)
            ),
        );
        BloonGraph { nodes }
    }

    pub fn children_of(&self, parent: BloonName) -> Vec<(BloonName, BloonNode)> {
        self.nodes[&parent]
            .children
            .iter()
            .map(|&(name, props)| {
                let mut base = self.nodes[&name].clone();
                if name != BloonName::DDT {
                    base.properties |= props & BloonProperty::CAMO;
                }
                (name, base)
            })
            .collect()
    }

    pub fn rbe(&self, name: BloonName) -> u32 {
        if name == BloonName::Red {
            1
        } else {
            println!("{:?}: {}", name, self.nodes[&name].hp);
            self.nodes[&name].hp
                + self
                    .children_of(name)
                    .iter()
                    .map(|&(name, _)| self.rbe(name))
                    .sum::<u32>()
        }
    }
}

#[test]
fn test_rbe() {
    let graph = BloonGraph::new();
    assert_eq!(graph.rbe(BloonName::Red), 1);
    assert_eq!(graph.rbe(BloonName::Blue), 2);
    assert_eq!(graph.rbe(BloonName::Yellow), 4);
    assert_eq!(graph.rbe(BloonName::Black), 11);
    assert_eq!(graph.rbe(BloonName::Rainbow), 47);
    assert_eq!(graph.rbe(BloonName::Ceramic), 104);
    assert_eq!(graph.rbe(BloonName::DDT), 816);
    assert_eq!(graph.rbe(BloonName::BAD), 55_760);
}
