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
  members: Option<Members>,
  tasks: Option<Tasks>,
  task_groups: Option<TaskGroups>,
}

type Person = String;
type Members = Vec<Member>;
type TaskAssignments = HashMap<Person, Option<Tasks>>;
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

    match self.members {
      Some(ref mut v) => v.push(new_memeber),
      None => self.members = Some(vec![new_memeber]),
    };

    if self.tasks.is_some() {
      self.set_up()
    }
  }

  pub fn add_task(&mut self, task: Task) {
    match self.tasks {
      Some(ref mut t) => t.push(task),
      None => self.tasks = Some(vec![task]),
    };

    if self.members.is_some() {
      self.set_up()
    }
  }

  fn set_up(&mut self) {
    let task_count = count_option_vec(&self.tasks);
    let member_count = count_option_vec(&self.members);
    let more_tasks_than_members = task_count >= member_count;

    // Reset task groups
    self.task_groups = None;

    // GROUP HAS MORE TASKS THAN PEOPLE
    if more_tasks_than_members {
      let can_be_evenly_divided = task_count % member_count == 0;

      if can_be_evenly_divided {
        let groups_of = task_count / member_count;

        let tasks = self
          .tasks
          .as_ref()
          .map(|tasks| tasks.chunks(groups_of))
          .unwrap();

        for chunck in tasks {
          match self.task_groups {
            Some(ref mut group) => group.push(chunck.to_vec()),
            None => self.task_groups = Some(vec![chunck.to_vec()]),
          };
        }
        assign_task_groups(self)
      } else {
        let groups_of = task_count / member_count;

        let (task_for_each, task_remaining) = self
          .tasks
          .as_ref()
          .map(|tasks| tasks.split_at(task_count - (task_count % member_count)))
          .unwrap();

        let mut task_chunks = task_for_each.chunks_exact(groups_of);

        for chunck in task_chunks.by_ref() {
          match self.task_groups {
            Some(ref mut group) => group.push(chunck.to_vec()),
            None => self.task_groups = Some(vec![chunck.to_vec()]),
          };
        }

        for (count, task) in task_remaining.iter().cloned().enumerate() {
          self
            .task_groups
            .as_mut()
            .map(|group| group[count].push(task));
        }

        assign_task_groups(self);
      }
    } else {
      // GROUP HAS LESS TASKS THAN PEOPLE
      for task in self.tasks.clone().unwrap() {
        match self.task_groups {
          Some(ref mut group) => group.push(vec![task]),
          None => self.task_groups = Some(vec![vec![task]]),
        }
      }

      assign_task_groups(self)
    }
  }

  pub fn rotate_tasks(&mut self) -> Result<(), &str> {
    let has_tasks_and_members = self.tasks.is_some() && self.members.is_some();

    if has_tasks_and_members {
      let members = self.members.as_ref().map(|members| members.iter()).unwrap();

      let mut assingments: Vec<Option<usize>> =
        members.map(|member| member.assigned_group).collect();

      assingments.rotate_left(1);

      // Assignment members new assignments
      for (pos, group_number) in assingments.iter().enumerate() {
        match self.members {
          Some(ref mut members) => members[pos].assigned_group = *group_number,
          None => {}
        }
      }

      Ok(())
    } else {
      Err("No tasks or members added")
    }
  }

  pub fn get_task_assignments(&self) -> Result<TaskAssignments, &str> {
    let has_tasks_and_members = self.task_groups.is_some() && self.members.is_some();

    if has_tasks_and_members {
      let mut task_assignments: TaskAssignments = HashMap::new();
      let members = self.members.as_ref().map(|members| members.iter()).unwrap();

      for member in members {
        let name = member.name.clone();
        let task_group = self
          .task_groups
          .as_ref()
          .map(|task_groups| match member.assigned_group {
            Some(group_pos) => Some(task_groups[group_pos].clone()),
            None => None,
          })
          .unwrap();

        task_assignments.insert(name, task_group);
      }

      Ok(task_assignments)
    } else {
      Err("No tasks or members added")
    }
  }
}

fn count_option_vec<T>(collection: &Option<Vec<T>>) -> usize {
  collection
    .as_ref()
    .map(|collection| collection.len())
    .unwrap()
}

fn assign_task_groups(group: &mut Group) {
  for group_position in 0..count_option_vec(&group.task_groups) {
    match group.members {
      Some(ref mut mem) => mem[group_position].assigned_group = Some(group_position),
      None => {}
    }
  }
}

pub fn create_tasks(n: usize) -> Tasks {
  let mut tasks = Vec::new();
  for num in 1..n + 1 {
    tasks.push(Task {
      name: format!("task {}", num).to_string(),
      details: format!("Do not forget to do task: {} it in the morning", num).to_string(),
    })
  }
  tasks
}

pub fn create_members(n: usize) -> Vec<Person> {
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
    for person in create_members(2) {
      group.add_member(person)
    }

    assert_eq!(group.members.unwrap()[1].name, "Member 2".to_string());
  }

  #[test]
  fn should_add_task_to_group() {
    let mut group = Group::new("Doing_team");
    for task in create_tasks(2) {
      group.add_task(task)
    }
    assert_eq!(group.tasks.unwrap()[0].name, "task 1".to_string())
  }

  #[test]
  fn should_setup_group_with_more_tasks_than_members_that_is_evenly_divisible() {
    let mut evenly_divisible_group = Group::new("Evenly Group");
    for task in create_tasks(6) {
      evenly_divisible_group.add_task(task)
    }
    for person in create_members(3) {
      evenly_divisible_group.add_member(person)
    }

    let compare: usize = 3;
    let length = evenly_divisible_group
      .task_groups
      .as_ref()
      .map(|group| group.len())
      .unwrap();
    assert_eq!(length, compare);

    let members = evenly_divisible_group.members.as_ref().unwrap();
    assert_eq!(members[2].assigned_group.unwrap(), 2);
  }

  #[test]
  fn should_setup_group_with_more_tasks_than_members_that_is_unevenly_divisible_which_results_to_one(
  ) {
    let mut unevenly_divisible_group = Group::new("UnEven Group");
    for task in create_tasks(6) {
      unevenly_divisible_group.add_task(task)
    }
    for person in create_members(4) {
      unevenly_divisible_group.add_member(person)
    }

    let compare: usize = 2;
    let first_group_length = unevenly_divisible_group
      .task_groups
      .as_ref()
      .map(|groups| groups[0].len())
      .unwrap();

    assert_eq!(first_group_length, compare);

    let members = unevenly_divisible_group.members.as_ref().unwrap();
    assert_eq!(members[0].assigned_group.unwrap(), 0)
  }

  #[test]
  fn should_setup_group_with_more_tasks_than_members_that_is_unevenly_divisible_which_results_to_more_than_one(
  ) {
    let mut unevenly_divisible_group = Group::new("UnEven Group");
    for task in create_tasks(15) {
      unevenly_divisible_group.add_task(task)
    }
    for person in create_members(4) {
      unevenly_divisible_group.add_member(person)
    }

    let compare: usize = 4;
    let first_group_length = unevenly_divisible_group
      .task_groups
      .as_ref()
      .map(|groups| groups[0].len())
      .unwrap();

    assert_eq!(first_group_length, compare);

    let members = unevenly_divisible_group.members.as_ref().unwrap();
    assert_eq!(members[0].assigned_group.unwrap(), 0)
  }

  #[test]
  fn should_setup_group_with_less_tasks_than_memebers() {
    let mut group_with_less_tasks = Group::new("More tasks group");

    for task in create_tasks(3) {
      group_with_less_tasks.add_task(task)
    }
    for person in create_members(4) {
      group_with_less_tasks.add_member(person)
    }

    let members = group_with_less_tasks.members.as_ref().unwrap();
    assert_eq!(members[2].assigned_group.unwrap(), 2);
    assert_eq!(members[3].assigned_group.is_none(), true);
  }

  #[test]
  fn should_rest_task_assignment_and_task_groups() {
    let mut group = Group::new("Group update");
    // 6 and 3 = 2 task each
    for task in create_tasks(6) {
      group.add_task(task)
    }
    for person in create_members(3) {
      group.add_member(person)
    }

    // 6 and 4 = 1 task each with two task groups having two
    group.add_member("Member 4".to_string());
    let task_groups = group.task_groups.unwrap();

    assert_eq!(task_groups[0].len(), 2);
    assert_eq!(task_groups[2].len(), 1);
  }

  #[test]
  fn should_rotate_task_assignments() {
    let mut group = Group::new("Group update");

    for task in create_tasks(6) {
      group.add_task(task)
    }
    for person in create_members(3) {
      group.add_member(person)
    }

    group.rotate_tasks().unwrap();

    let members = group.members.unwrap();
    assert_eq!(members[0].assigned_group, Some(1))
  }

  #[test]
  fn should_not_rotate_tasks_with_no_members_or_tasks() {
    let mut empty_group = Group::new("Empty Group");

    assert_eq!(empty_group.rotate_tasks().is_err(), true);
  }

  #[test]
  fn should_create_task_assignment() {}
}
