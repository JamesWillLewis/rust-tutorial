
#[derive(Debug)]
struct Agent {
   id: String,
   name: String,
   time_step: i32,
   active: bool
}
impl Agent {
   fn tick(&mut self) {
      self.time_step += 1;
   }
}



fn main() {
   let mut agent_1 = new_agent(String::from("agent_1"), String::from("Bob Ross"));

   agent_1.time_step += 1;

   print_agent(&agent_1);

   // moves .name (doesn't copy) because its data is on heap
   let mut agent_2 = Agent {
      id: String::from("agent_2"),
      ..agent_1
   };
   agent_2.time_step += 5;

   print_agent(&agent_2);

   // println!("Name of agent 1 is {}", agent_1.name) -- this fails because name was moved

   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);

   let unit_type = UnitType;

   println!("Printing the whole struct: {:#?}", agent_2);

   print_agent(&agent_2);
   agent_2.tick();
   agent_2.tick();
   agent_2.tick();
   print_agent(&agent_2);

}

fn print_agent(agent: &Agent) {
   println!("{} is on time step {} with name {}", agent.id, agent.time_step, agent.name);
}

fn new_agent(id: String, name: String) -> Agent {
   // shorthand init syntax
   Agent {
      id,
      name,
      time_step: 0,
      active: true
   }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct UnitType;