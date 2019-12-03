# Contributing

The `Dothereum` project is an _Open Open Source Project_.

Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project. Code and repository governance is documented in [dothereum/coordination](https://github.com/dothereum/coordination) and everyone is encouraged to participate.

## Rules

There are a few basic ground-rules for contributors including the maintainers and administrators of the project:

- No `--force` pushes or modifying release branch history in any way. If you need to rebase, ensure you do it in your own repository.
- Feature branches, prefixed with a short name moniker, e.g., `s1-my-feature`, must be used for ongoing work.
- All modifications must be made in a pull-request to solicit feedback from other contributors.
- A pull-request must not be merged until CI has finished successfully.
- Contributors should adhere to the [Substrate Style Guide](https://wiki.parity.io/Substrate-Style-Guide).

## Merge Process

Merging pull requests once CI is successful:

- A PR needs to be reviewed and approved by project maintainers unless:
  - it does not alter any logic, e.g., comments, dependencies, docs, then it may be merged by its author once CI is complete.
  - it is an urgent fix with no large change to logic, then it may be merged after a non-author contributor has approved the review once CI is complete.
- Once a PR is ready for review please add the `pr0-review` label. Generally PRs should sit with this label for 48 hours in order to garner feedback. It may be merged before if all relevant parties had a look at it.
- If the first review is not an approval, swap `pr0-review` to label `pr5-grumble` to indicate that the PR has received some feedback, but needs further work. For example. `pr3-inprogress` is a general indicator that the PR is work in progress. Once the work is done, change the label back to `pr0-review`. You might end up swapping a few times back and forth to climb up the `prX` label group. Once a PR is `pr9-good` it is ready to merge.
- PRs that break the external API or change the runtime logic must be tagged with `z6-releasenotes`.
- No PR should be merged until all reviewer's comments are addressed.

Reviewing pull requests:

- When reviewing a pull request, the end-goal is to suggest useful changes to the author. Reviews should finish with approval unless there are issues that would result in:
  - Buggy behavior.
  - Undue maintenance burden.
  - Breaking with house coding style.
  - Pessimization, i.e., reduction of speed as measured in the projects benchmarks.
  - Feature reduction, i.e., it removes some aspect of functionality that a significant minority of users rely on.
  - Uselessness, i.e., it does not strictly add a feature or fix a known issue.

Reviews may not be used as an effective veto for a PR because:

- There exists a somewhat cleaner/better/faster way of accomplishing the same feature/fix.
- It does not fit well with some other contributors' longer-term vision for the project.

## Helping out

We use _labels_ to manage PRs and issues and communicate state of a PR. Please familiarize yourself with them. Furthermore we are organizing issues in _milestones_.

## Releases

Declaring formal releases remains the prerogative of the project maintainers.

## Changes to this arrangement

This is an experiment and feedback is welcome! This document may also be subject to pull-requests or changes by contributors where you believe you have something valuable to add or change.

## Heritage

These contributing guidelines are modified from the _Open Open Source Project_ guidelines [for the Level project](https://github.com/Level/community/blob/master/CONTRIBUTING.md).
