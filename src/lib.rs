use std::collections::HashMap;

// Task Assigments
#[derive(Debug)]
pub struct Member {
  name: Person,
  assigned_group: Option<usize>,
}

// Task
#[derive(Debug, Clone)]
pub struct Task {
  name: String,
  details: String,
}

// Group
pub struct Group {
  pub name: String,
  pub members: Option<Members>,
  tasks: Option<Tasks>,
  task_groups: Option<TaskGroups>,
}

type Person = String;
type Members = Vec<Member>;
type TaskAssignments = HashMap<Member, Tasks>;
type Tasks = Vec<Task>;
type TaskGroups = Vec<Tasks>;

impl Group {
  pub fn new(group_name: &str) -> Self {
    Group {
      name: String::from(group_name),
      members: None,
      tasks: None,
      task_groups: None,
    }
  }

  pub fn add_member(&mut self, member: Person) {
    let new_memeber = Member {
      name: member,
      assigned_group: None,
    };

    match &mut self.members {
      Some(v) => v.push(new_memeber),
      None => self.members = Some(vec![new_memeber]),
    };
  }

  pub fn add_task(&mut self, task: Task) {
    match &mut self.tasks {
      Some(t) => t.push(task),
      None => self.tasks = Some(vec![task]),
    };
  }
}

pub fn create_tasks_data(n: usize) -> Tasks {
  let mut tasks = Vec::new();
  for num in 1..n + 1 {
    tasks.push(Task {
      name: format!("task {}", num).to_string(),
      details: format!("Do not forget to do task: {} it in the morning", num).to_string(),
    })
  }
  tasks
}

pub fn create_member_data(n: usize) -> Vec<Person> {
  let mut members = Vec::new();
  for num in 1..n + 1 {
    members.push(format!("Member {}", num))
  }
  members
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_add_member_to_group() {
    let mut group = Group::new("Doing_team");
    for person in create_member_data(2) {
      group.add_member(person)
    }

    assert_eq!(group.members.unwrap()[1].name, "Member 2".to_string());
  }

  #[test]
  fn should_add_task_to_group() {
    let mut group = Group::new("Doing_team");
    for task in create_tasks_data(2) {
      group.add_task(task)
    }
    assert_eq!(group.tasks.unwrap()[0].name, "task 1".to_string())
  }
