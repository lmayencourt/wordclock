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

## FW 1: Configuration validity flag
In the context of implementing the persistent storage for the *WordClock* configuration,
facing the need to store a boolean value in persistent memory,
I decided to use a String value "0" and "1",
and neglected a dedicated boolean storage API,
to achieve a simple implementation, using the already implemented String storage API,
accepting a possible higher memory footprint of the boolean value in memory.

## FW 2: Directory layout for OTA images
In the context of releasing the OTA images for the new hardware v2 and Rust implementation,
facing the need to keep the hardware v1 and firmware v1.x.x/v2.x.x OTA working,
I decided to keep the hardware v1 OTA image at the root, and move the hardware v2 OTA image into a dedicated `ota-image/hardware-v2/` folder,
and neglected a dedicated branch `released-v2` or equivalent,
to achieve a backward compatible layout, and keep the single, easier to maintain release branch,
accepting that the hardware v1 OTA image stay for now in the root of the repository, and a migration process will be needed to move them to `ota-image/hardware-v2`.