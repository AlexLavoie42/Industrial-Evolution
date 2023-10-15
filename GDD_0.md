# Industrial Evolution


## Basic Concepts

- Main scene = Factory (prepare for scaling to multiple factories)

- Items
    - Goods
    - Resources
    - Equipment (form of Good?)
- Assemblies
    - Transform Resources into Goods
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
    - Increase complexity with each path collision
    - Each worker can handle a certain amount of complexity until they begin to have a % chance of error. Workers can handle more complexity as they become more skilled.
    - Errors can cause relevant Resources, Goods, Assemblies or Equipment to break.
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
        - Can be automated with engines but difficult & limited (spinning poles)
    - Thermal (temperature)
        - Used for Assemblies that require heat
        - Produced by assemblies using resources + mechanical power
        - Must be produced next to assembly
    - Electrical (GJ)
        - Used for electric Assemblies
        - Purchased externally, requires advanced assemblies to transport & use


## Prototype

1 Good per Power stage

- Resources
    - Wood
    - Pulp
- Goods
    - Paper
- Assemblies
    - Pulp Mill
    - Paper Machine
    - (Market Interaction) Receivables?
- Worker
    - Path setting between assemblies
    - Interact with assemblies
    - Hold configurable amount of items and move them between assemblies
- Market
    - Buy / Sell Resources
    - Sell Goods
    - Configurable prices
    - Configurable market events from player interaction
    - Configurable random market events


## Economy
- Money is the central source of everything in the game.
- Factory will require upkeep depending on size, assemblies & workers.

- Advanced per Resource / Good pricing
    - Each item can recieve upward & downward market force events
    - Each event will push the price by a random amount upward or downward respectively
    - Event are weighted based on the size of the market for each item, so events in smaller markets will have a larger impact.
- External Market events will be random / scripted to start
- Selling or Buying will create a market event for that item


## Core Gameplay Loop
The goal of this prototype will be to judge the most basic mechanics and determine if they are fun enough to be played for a long time with little polish. The core game mechanics will only include the bare minimum amount of mechanics & depth to be considered fun.

- Unlock new Assembly
- Start producing new Good using old Goods and Resources
- Sell Good for profit
- Hire Workers and use them to automate new assembly
- Scale Assembly with automation

## Random Thoughts

- Assembly by-products
- Multi-step production (Goods cannot be bought (in small quantities manually?), forcing multi-step production)
- Converging & diverging production lines
- Market:
    - Slightly random artificial price increases
    - "Demand" for Goods decreases as you unlock new ones

- Different classes of Items? Could provide unique challenges. e.g. illegal, volatile, etc


## Story / Feelings Ideas

- Start poor, daily living expenses that heavily cut into profits. Feeling: Survival, will encourage short term decisions for profit e.g Papers please
- Beginning to expand, survival is no longer a priority. Need some kind of longer term objective at this point. Feeling: Problem solving, accomplishment. e.g Factorio

- World highlights poverty, and has the player interact with it throughout
- Interaction with workers while trying to maximize profits for goals
