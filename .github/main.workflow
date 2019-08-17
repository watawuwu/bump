workflow "New workflow" {
  on = "push"
  resolves = ["run"]
}

action "run" {
  uses = "run"
}
