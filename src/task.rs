pub fn create_task(command: CreateTaskCommand) -> Result<(), String> {
    let mut task = Task::new(command.name, command.group);
    task.save()?;
    Ok(())
}