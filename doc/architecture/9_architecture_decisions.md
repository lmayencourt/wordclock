# Architecture decisions

All decisions are recorded according to the [Y-statement](https://socadk.github.io/design-practice-repository/artifact-templates/DPR-ArchitecturalDecisionRecordYForm.html) [ADR](12_glossary.md#ADR) template.

````
In the context of <use case/user story u>,
facing <concern c>
we decided for <option o1>
(and against <option o2,o3>)
(because <>)
to achieve <quality q>,
accepting <downside d>.
````

## PCB 1: LEDs matrix for display
In the context of designing a LEDs matrix for the *WordClock* display,
facing the need to choose LEDs component,
I decided to use WS2812 LEDs strip,
and neglected single LEDs or WS2812 LEDS,
to achieve low-cost and simple PCB assembly,
accepting the bigger dimension constraints of the LEDs strip.

## PCB 2: LEDs matrix routing
In the context of routing the LEDs matrix for the *WordClock* display,
facing the need to place the WS2812 LEDs strips on the [PCB](12_glossary.md#PCB),
I decided to keep the same layout as the *WordClock* hardware v1.0 (S like routing of the LEDs strips),
and neglected a "straight" layout,
to achieve a unified design across hardware iteration, limiting the number of variant,
accepting a slightly more complex LEDs matrix driver (half of the LEDs strips are inverted).
