# Building Block View

This chapter explains the building blocks of the *WordClock* system, and its decomposition in sub-system.

## White box of the *WordClock*
![level_1](../uml/exported/building_blocks_lvl_1.png)

| Name | Responsibility |
| - | - |
| Application | provides the high-level system behavior. |
| Platform abstraction | decouples the Application implementation from the platform specific hardware. | 
| Platform | contains all the hardware specific features. |
| Real Time Clock | is the hardware component responsible to keep track of the time when the system is powered off. |
| Led Matrix | is the hardware component used as display. |

![level_2](../uml/exported/building_blocks_lvl_2.png)