# Introduction and goals
*WordClock* is a hobby project, aiming to create a nice looking time keeping art work.

The secondary goal is to have a project to experiment with various technologies like Rust, laser cutting, 3d printing, and various engineering skills like software architecture, software requirements, Test-Driven Development and documentation.

This document is not limited to software architecture, but considers all the work realized by the *Embedded engineer*, include [PCB](12_glossary.md#PCB) design.

## Requirements overview
Essential features:
- Display current time, using your favorite Swiss-German dialect.
- Set system time, manually or automatically.
- Keep time accurately.
- Disable display during the night, if wanted.

## Quality goals
| ID | Quality | Motivation |
| - | - | - |
| QG1 | Reliability | The system shall function reliably under any circumstances. |
| QG2 | Autonomous | The system shall provide long up-time without human intervention. |
| QG3 | Testability | The architecture should allow easy testing of all main building blocks. |

## Stakeholder
| Stakeholder | Goal, Intentions |
| - | - |
| Owner | Want a nice time keeping device, that display the time in her favorite Swiss-German dialect. |
| Embedded engineer | Develop the electronic hardware and software that make the *WordClock* does its things; Learn and apply new software development skills. |
| Wood engineer | Manufacture the *WordClock* physical parts; Provides feedbacks and constraints on the *WordClock* hardware and software. |

## Use case
Developer:
1. Release new version of the firmware.
2. Install the firmware on the system.
3. Address *User* reported issue.

User:
1. Read time from system.
1. Configure the time of the system.
2. Configure the dialect of the system.
3. Configure Wi-Fi setup for automatic time setting.
4. Configure *Night time*, where the clock is not displaying the time.
5. Report issue to developer.
6. Update to latest released version of the firmware, without the need of advanced technical knowledge.

extra:
- Bluetooth configuration? not yet, but can be an upgrade.
- Chose color of the time? yes!
