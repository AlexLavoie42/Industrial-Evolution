# Industrial Evolution


## Basic Concepts

- Main scene = Factory (prepare for scaling to multiple factories)

- Items
    - Goods - Can be sold in bulk but buying is restricted?
    - Resources - Can be bought & sold in bulk
    - Materials - Can't be bought or sold
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

~~1 Good per Power stage~~
2-3 Goods per power stage, allows testing economy management

- Resources
    - Wood
    - Pulp
    - Lumber
    - Copper ore
    - Copper
    - Iron ore
    - Wraught Iron
    - Steel
    - Stainless Steel

- Goods
    - Paper
    - Books
    - Furnature
    - Tools
    - Pipes
    - Wire

- Assemblies
    - Wood chipper
    - Pulp Mill
    - Paper Press
    - Drying Rack
    - Lumber Mill
    - Woodworking shop

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
    - Larger price gaps (base price - current price) increase the amount of market events in a particular direction
- External Market events will be random / scripted to start
- Selling or Buying will create a market event for that item
- Simulating competition & have player interact (sabotage / cooperate)

### Imports
- Automatically purchase, ship & store resources
- Configurable refill limit & shipment size
- Configurable limit price?
- Limited placement (Factory walls?)

### Export
- Automatically sell goods
- Configurable limit price?
- Limited placement (Factory walls?)

### External Power
- Automates assembly production for a high upfront cost + upkeep cost
- Limited placement (Factory walls?)
- Power can be transported to assemblies
    - Mechanical = Shaft
    - Thermal = Steam? Conductive? Limited transportation (Focus on resources e.i. Coal)?
    - Electricity = Wires

### Conveyors
- Slowly moves items automatically
- Items must be manually loaded & removed by player or worker

## Core Gameplay Loop
The goal of this prototype will be to judge the most basic mechanics and determine if they are fun enough to be played for a long time with little polish. The core game mechanics will only include the bare minimum amount of mechanics & depth to be considered fun.

- Unlock new Assembly
- Start producing new Good using old Goods and Resources
- Sell Good for profit
- Hire Workers and use them to automate new assembly
- Scale Assembly with automation

Learnings:
- Easy to fall into trap player having nothing to do while waiting for production.
    - Make sure the player is always the bottleneck to keep the game engaging
    - Worker should be very expensive to encourage manual labour from the player
    - Manual production should be engaging
        - Minigames?
        - Interacting with workers (complexity?)
- UI is important for prototype. Without it playtesting is frustrating & difficult
- Start the game without automation as a tutorial
- Introduce automation to the player once step at a time
- Slowing monetary progression
    - Balancing dynamic economy will be really hard
 
- Base gameplay loop is not engaging enough
    - Worker setup/automation doesnt scale
    - Players manually doing production is not fun enough. Minigames help but are band-aid

    - Focus on intensity of time crunch during day cycle. Make enough money to survive. Keep player as the bottleneck either directly in production or managing/fixing/optimizing workers.
    - Stardew valley style, try to fill up output before day is over.
    - End of day lets player relax.
    - Selecting resources to be bought for the next day. Unused resources incur storage fees.
    - Core loop is a cycle between intense factory production time crunch, and end of day relief. Gives players breathing room and time to plan.

- Managing workers seems fun, lean into it
    - Micro managing for greater efficiency
        - More dynamic management (follow / copy player? One time tasks? )
    - Training & skills
    - Worker needs & happiness
    - Always leave room for improvement
        - Unlock new possibilies with worker skills. Force the player to rethink certain designs
        - Multi tasking jobs. As skill increases worker gets smarter when selecting tasks.

- Give the player more to do with their money
    - Balance around player staying poor most of the game?
    - Casino / gambling
    - Houses / luxury? (Maybe give boosts to player?)
    - Player skills?
    - External investments
        - Resource gathering (mining, forestry, etc.)
        - Power generation
        - Stocks?

- Bring in electricity at beginning?
    - Powering manually seems weird and boring
    - Would introduce more complexity right away

- Economy should revolve around forcing player to shift production often
    - Stops the player from just idling to make money
    - Use economy to force players into multi step production

- Night time is 1/2 the game. Using a boring UI will kill the game.
    - Turn the night UI into an interactable area
    - Give just profits UI as a transition
    - House where you interact with items to select living costs & orders
    - Hire workers at night
    - Check market stats (supply & demand, price over time)
    - Order assemblies at night? Place from inventory?
    - Research?

- Running out of resources is dumb
    - Order more during day
        - Delay?
        - Extra cost?
    - Balance
        - Slow down production of goods
            - Each good production should involve solving a small puzzle. Worker should just make this easier.
        - Balance order sizes
            - Min order size / batches?
            - Import & export container sizes
    - Rework ordering
        - Different phases of day? Import shipments get staggered over time
        - Auto purchase?

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
