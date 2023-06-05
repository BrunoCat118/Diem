Workflows/ contain [github actions](https://github.com/features/actions) that can run on specific events.

Below is a list of all actions implemented in this directory:

* `hyperjump-*`. These are backend hyperjump workflows to trigger specific
  actions that come via hyperjumps routed through `repository_dispatch`
  triggers.
* [dep-summaries](dep-summaries.yml). This workflow monitors dependency
  changes to special subsets and flags them in the PR.

### Change Log - May 2023
#### GitHub Action Fix for ci-test ci-post-land and daily workflows
#### XL runners are removed and adding self hosted runners untill we get approved for the Large Runners
#### PR to fix the issues at https://github.com/diem/diem/pull/10863

#### ci-test fix
* Created self-hosted runners
* Order changes to the jobs so that all jobs does not run at same time
* adding the runs-ons section of jobs to self-hosted runner names ( like testnet-runner for amazon linux and testnet-runner-ubuntu for ubuntu based jobs )
* remove all forge related jobs
* adding always() in the if condition to allow other jobs to run if previous jobs fails
* adding below step in jobs where remove permission were an issue in new self-hosted runners
- name: Chown user
  run: |
    sudo chown -R $USER:$USER $GITHUB_WORKSPACE
* adding steps to cleanup workspaces in self-hosted runner using ``` docker system prune ``` command and ``` docker volume prune --all -f ```

#### ci-post-land fix
* adding the runs-ons section of jobs to self-hosted runner names
* remove all forge related jobs
* adding steps to cleanup workspaces in self-hosted runner using ``` docker system prune ``` command

#### daily fix
* url fixes
* self-hosted runner changes
* clean up steps
