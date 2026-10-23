use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use shared::task::Task;

pub struct Graph {
    deps: HashMap<String, Vec<String>>,
    dependents: HashMap<String, Vec<String>>,
}
pub fn build_graph(tasks: &[&Task], ids: &[String]) -> Result<Graph, String> {
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

    for id in ids {
        if deps.contains_key(id) {
            return Err(format!("Duplicate task ID: {}", &id));
        }

        let task: &Task;
        if let Some(index) = ids.iter().position(|item| item == id.deref()) {
            task = tasks[index];
        } else {
            return Err("Task not found".into())
        }

        deps.insert(
            id.clone(),
            task.depends.clone().unwrap_or_default(),
        );

        dependents.insert(id.clone(), Vec::new());
    }

    for id in ids {
        let task_deps = deps.get(id).unwrap();

        for dep_id in task_deps {
            if !deps.contains_key(dep_id) {
                return Err(format!(
                    "Task '{}' depends on unknown task '{}'",
                    id, dep_id
                ));
            }

            dependents
                .get_mut(dep_id)
                .unwrap()
                .push(id.clone());
        }
    }

    Ok(Graph { deps, dependents })
}

pub fn topological_order(graph: &Graph) -> Result<Vec<String>, String> {
    let mut remaining: HashMap<String, usize> = graph
        .deps
        .iter()
        .map(|(task_id, dependencies)| {
            (task_id.clone(), dependencies.len())
        })
        .collect();

    let mut ready: VecDeque<String> = remaining
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(task_id, _)| task_id.clone())
        .collect();

    let mut order = Vec::with_capacity(remaining.len());

    while let Some(task_id) = ready.pop_front() {
        order.push(task_id.clone());

        if let Some(dependents) = graph.dependents.get(&task_id) {
            for dependent_id in dependents {
                let count = remaining
                    .get_mut(dependent_id)
                    .expect("dependent should exist in graph");

                *count -= 1;

                if *count == 0 {
                    ready.push_back(dependent_id.clone());
                }
            }
        }
    }

    if order.len() != graph.deps.len() {
        return Err("Dependency cycle detected".to_string());
    }

    Ok(order)
}