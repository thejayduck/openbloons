use bloon::{BloonGraph, BloonName, BloonProperty};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Drawable};
use ggez::timer::check_update_time;
use ggez::{input, timer, Context, ContextBuilder, GameResult};

pub mod bloon;

#[derive(Debug, PartialEq)]
enum DamageResult {
    Popped,
    Children(Vec<BloonInstance>),
    Nothing,
}

#[derive(Debug, Clone, PartialEq)]
struct BloonInstance {
    pub initial: BloonName,
    pub current: BloonName,
    pub hp: u32,
    pub properties: BloonProperty,
    pub travel: f32,
}

impl BloonInstance {
    pub fn new(graph: &BloonGraph, name: BloonName, properties: BloonProperty) -> Self {
        let node = &graph.nodes[&name];
        BloonInstance {
            initial: name,
            current: name,
            hp: node.hp,
            properties: node.properties | properties,
            travel: 0.0,
        }
    }

    pub fn take_damage(&mut self, graph: &BloonGraph, damage: u32) -> DamageResult {
        let mut damage = damage;
        // Should we pop the extras?
        let mut extras = Vec::new();
        while damage >= self.hp {
            damage -= self.hp;
            self.hp = 0;

            let children = graph.children_of(self.current);
            if children.is_empty() {
                return DamageResult::Popped;
            }

            let this = &children[0];
            self.current = this.0;
            self.hp = this.1.hp;

            extras.extend(children[1..].iter().map(|child| {
                let mut bloon = BloonInstance::new(graph, child.0, child.1.properties);
                bloon.travel = self.travel;
                bloon
            }));
        }

        self.hp -= damage;

        if extras.is_empty() {
            DamageResult::Nothing
        } else {
            DamageResult::Children(extras)
        }
    }
}

#[test]
fn popping() {
    let graph = BloonGraph::new();
    let mut bloon = BloonInstance::new(&graph, BloonName::Green, BloonProperty::empty());

    assert_eq!(bloon.take_damage(&graph, 2), DamageResult::Nothing);
    assert_eq!(bloon.current, BloonName::Red);

    let mut bloon = BloonInstance::new(&graph, BloonName::Ceramic, BloonProperty::empty());

    assert_eq!(bloon.take_damage(&graph, 8), DamageResult::Nothing);
    assert_eq!(bloon.current, BloonName::Ceramic);

    assert_eq!(
        bloon.take_damage(&graph, 4),
        DamageResult::Children(vec![
            BloonInstance {
                initial: BloonName::Rainbow,
                current: BloonName::Rainbow,
                hp: 1,
                properties: BloonProperty::empty(),
                travel: 0.0
            },
            BloonInstance {
                initial: BloonName::Zebra,
                current: BloonName::Zebra,
                hp: 1,
                properties: BloonProperty::empty(),
                travel: 0.0
            },
            BloonInstance {
                initial: BloonName::White,
                current: BloonName::White,
                hp: 1,
                properties: BloonProperty::empty(),
                travel: 0.0
            }
        ])
    );
    assert_eq!(bloon.current, BloonName::Black);
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Open Bloons", "Shinaosu")
        .build()
        .unwrap();

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}

struct Game {
    graph: BloonGraph,
    bloons: Vec<BloonInstance>,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            graph: BloonGraph::new(),
            bloons: Vec::new(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::ticks(ctx) % 100 == 0 {
            self.bloons.push(BloonInstance::new(
                &self.graph,
                BloonName::Yellow,
                BloonProperty::empty(),
            ));
        }
        if input::keyboard::is_key_pressed(ctx, KeyCode::Space) {
            if let Some(first) = self.bloons.first_mut() {
                match first.take_damage(&self.graph, 1) {
                    DamageResult::Popped => {
                        self.bloons.remove(0);
                    }
                    DamageResult::Children(children) => {
                        self.bloons.extend_from_slice(&children);
                    }
                    DamageResult::Nothing => {}
                }
            }
        }
        while check_update_time(ctx, 60) {
            for bloon in &mut self.bloons {
                bloon.travel += 100.0 * 1.0 / 60.0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        for bloon in &self.bloons {
            graphics::Mesh::new_ellipse(
                ctx,
                DrawMode::fill(),
                glam::Vec2::ZERO,
                12.0,
                15.0,
                0.01,
                match bloon.current {
                    BloonName::Red => Color::RED,
                    BloonName::Blue => Color::BLUE,
                    BloonName::Green => Color::GREEN,
                    BloonName::Yellow => Color::YELLOW,
                    _ => todo!(),
                },
            )?
            .draw(ctx, DrawParam::new().dest(glam::vec2(bloon.travel, 200.0)))?;
        }
        graphics::present(ctx)
    }
}
