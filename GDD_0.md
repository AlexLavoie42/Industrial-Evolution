# Industrial Evolution


## Basic Concepts

- Main scene = Factory (prepare for scaling to multiple factories)

- Items
    - Goods
    - Materials
    - Equipment (form of Good?)
- Assemblies
    - Transform Materials into Goods
    - Requires some form of "Work" to operate
    - Reliability, determines how likely the assembly will break when used.
- Work
    - Used to power Assemblies
    - Can be produced by a Worker
    - Can be automated in various ways
    - Different forms of Work
        - Based on forms of Power
        - Workers require some special equipment for some forms
- Worker
    - Can be given instructions to automate production
    - Easiest form of automation but most upkeep & least reliable.
    - Holds a various amount of Items.
    - Given a Path known as a "Job" that they will repeat.
    - Move items between assemblies, provide work.
    - Job complexity is measured in amount of steps weighted by the configured complexity for each step (Move item = Path Length, Provide Work = N).
    - Each worker can handle a certain amount of complexity until they begin to have a % chance of error. Workers can handle more complexity as they become more skilled.
    - Errors can cause relevant Materials, Goods, Assemblies or Equipment to break.
    - Errors can also cause injury which gives penalties.
    - Errors can rarely cause death (increases with Job "danger"? Could be similar but separate from complexity)
    - Lose worker on death & other penalties (Fines, restrictions, loss condition?)
- Power
    - Allows the automation of Work
    - Special assemblies to convert Power to Work?
    - Takes various forms

    - Mechanical (torque)
        - Used for basic Assemblies
        - Can be produced by Worker with no equipment
    - Thermal (temperature)
        - Used for Assemblies that require heat
        - Requires special equipment for Workers to produce
    - Electrical (GJ)
        - Used to electric Assemblies
        - Can not be produced by Workers


## Economy
- Money is the central source of everything in the game.
- Factory will require upkeep depending on size, assemblies & workers.

- Advanced per Material / Good pricing
    - Each item can recieve upward & downward market force events
    - Each event will push the price by a random amount upward or downward respectively
    - Event are weighted based on the size of the market for each item, so events in smaller markets will have a larger impact.
- External Market events will be random / scripted to start
- Selling or Buying will create a market event for that item