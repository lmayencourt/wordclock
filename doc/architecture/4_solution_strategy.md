# Solution Strategy

| Quality goal | Achieved by | Rational |
| - | - | - |
| Reliability | Custom [PCB](12_glossary.md#PCB) design for the system hardware. | The electronic components are assembled on a professionally manufactured circuit board. No manual wiring is needed. |
| | [Rust programming language for the system software](8_crosscutting_concepts.md#rust-std-environment-for-esp32). | Use a system language that focus on correctness and provides native support for testing. |
| | [Documented software anomalies](8_crosscutting_concepts.md#document-software-anomalies). | Anomalies are documented and can be addressed later on by developers. |
| Autonomous | External [RTC](12_glossary.md#rtc) with a backup battery. | Local time is keep by a dedicated, high precision circuit. The backup battery allow the circuit to keep the time even when the system is not powered. |
| | Automatic time synchronization with [NTP](12_glossary.md#ntp). | No need for the user to enter the time manually, even after power lose. |
| Testability | [TDD for software module](8_crosscutting_concepts.md#test-driven-development-for-embedded-system).| Increase confidence of software working as expected. |
| | Follow [test-double](https://en.wikipedia.org/wiki/Test_double) names for testing. | Follow a standard, that helps developers to understand the tests structure and behavior. |
