# Design notes on the format of Panel configuration files
The overall goal is to enable users to write configurations that express a world
map made of background panels.

### Support for non-cartesian geography
Panels are just nodes on a virtual graph, and they aren't placed in a single
coordinate system.  As such, a Creator could construct a non-cartesian world.

A - B - C
|       |
D ----- E

If all panels are the same size, it could be faster to travel from A to E by
going through D instead of going through B and C.  If D is a really large panel,
and B and C are small, it might be quicker the other way.

*Define: Panel*  A single "map area" which is combined with others like it to
form an overall "world."  Example: If you look at a map of Hyrule from the first
Zelda game, each of those rectangles is a "Panel."

## Attributes of Panels

### Size
*Panels are all rectangles, but size may vary from Panel to Panel.*  Some Panels
can be long and thin while some might be square.  They do not necessarily fill
the screen, or fit in the screen.

`Size of a Panel is determined by the size of its background asset`

*When size units are necessary, they are specified in 'mu' (map units).*
When specifying size or position units, use "Map Units" (mu) instead of pixels.
Pixel values would change with asset resolution, causing all the size/position
specs to also change.  Map units, defined as a ratio to pixels, fix the problem.

`Creators specify how many pixels per mu for their map; default is 1:1`

### Connectivity
*Panels connect to each other.*  A player moves between panels as they move
around the world.  The connectivity between Panels is important because it lets
us know what assets we can pre-cache.

`Panels have a name that can be referenced elsewhere in the config file`
`Edges (Transitions) between panels are specified in the config file`

*Neighboring Panels might be determined at runtime.*  Creators might want a
maze, or an "endless desert" that repeats forever in one direction but returns
to another Panel in the other direction.

### Assets
*Panels have a single background asset.*  Each panel has a background asset
that gets rendered behind all other game assets.

`Creators specify the background asset using the 'background' attribute`

*Panels have other associated assets.*  There are more than just the background
panel on the screen at once, and we can know which other assets are likely to be
on the screen based on which panel the player is in.

*Panels are often a part of biomes.*  Those associated assets before often come
in the form of sets that are shared between panels that are similar.

### Transitions
*Animations vary between Panel Transitions.*  Sometimes you want to slide in the
direction of the adjacent Panel, sometimes you want to fade out, or warp, etc.

`Edges to other Panels must also specify the Transition to use`
`A set of common Transitions is provided; e.g. SLIDE_LEFT, FADE, WARP`

*Not all adjacent Panels have Transitions*  A large area, such as a town, might
be made of many Panels because it's too expensive to load the assets for the
whole Panel, or just because the Creator wants it that way.  In such cases, hard
Transitions between the Panels wouldn't exist, and we'd show multiple of them
on the screen at the same time.

`An Edge can specify CONTINUOUS Transition + a pixel offset for rendering`

*Transitions between Panels vary according to game logic.*  Areas get unlocked,
some transitions have cinematic value once but not again, etc.  "Conditional
Transitions" provide this functionality.  These Transitions only trigger when
a game flag is set true.

`Conditional Transitions can be specified with a flag name`
`Programmers can call a library function to set the flag on/off`

*Trigger areas for Transitions vary in shape and size.*  Transition triggers
might be arbitrary shapes, lines across the Panel, closeness to the edge, etc.

`Edges must specify the Transition trigger area`
`A set of common Triggers is provided; e.g. RECTANGLE, EDGE, LINE, CIRCLE`
`Custom Trigger areas can be specified with a mask asset`

*Transitions and Triggers can be bi-directional.*  Often the Transition from
Panel A to Panel B goes the other way too.  If the attribute "bi-directional"
is set to "true" then a second Transition is automatically created with mirrored
versions of the Transition animations and Trigger bounds.

`Setting 'bi-directional' to true will generate a mirrored Transition`
