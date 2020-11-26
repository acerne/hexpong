# Ideas

## Game types
- [ ] Single player - player vs nothing
  - *Goal is to simply clear the blocks*
  - *Possibly reduce the number of bars and turn them into walls, to reduce the difficulty.*
- [ ] Multi-player - same keyboard
  - *Ball remembers the last player that touched it.*
  - *Goal of the game is to clear as many objects.*
  - *Lost balls are counted with a negative score to a player that lost it.*
- [ ] Single player - player vs AI
  - *Same rules as multi-player.*
  - *AI controlls the other set of bars.*
- [ ] Difficulty
  - *Ball speed, bar size, bar speed, number of bars/walls.*
- [ ] Bar shape & physics
  - *Bar should be such that it is able to affect the physics of the ball bouncing away.*
  - *Either by having a shape that is able to redirect the ball, or by traction, that is able to pull and redirect the ball on touch.*
  - *Another option is to add slight rotation to the bar when it moves. Should probably need acceleration and analogue input, which makes it mouse only option.*
  - *Without it there is a possibility of a deadlock, where ball moves perpendicular to the bar and no obstacles are in it's path.*
  - *Perhaps there could be a small area in the center of the bar, where ball could be "caught" and redirected by adjusting the departure angle, indicated by an arrow*
- [ ] Grid size and shape
  - *Something to be defined in a level*

## Blocks

### Normal blocks
- [x] basic block
  - *Single touch destroys it, nothing more to it.*
- [x] multi-hit blocks 
  - *Blocks that requre multiple ball hits to be destroyed.*
  - *With each hit the block reduces the color, indicating amount of hits required.*
  - *Maybe marked with a number drawn over the block.*
- [x] immortal blocks
  - *Non-destructible blocks, which present a permanent obstacle on the playing ground.*

### Special blocks
These have a special action or feature, that affects the gameplay instantly on hit.
- [ ] explosive block
  - *Block that destroys blocks in it's radius.*
  - *Should it destroy immortal blocks as well? Does that make them mortal?*
  - *Chain reaction if multiple explosive blocks are within the blast radius.*
- [ ] owned blocks
  - *Block can only be destroyed by the player of the corresponding color.*
- [ ] new-ball block
  - *Block spawns a new ball or multiple new balls*
  - *Balls spawned should be of the same color and properties as the ball that touched the block.*
  - *Should balls be spawned at last bar location or at the block location?*
- [ ] toggling block
  - *Ball once bounces, once passes through the block, toggling if it will bounce or pass with each touch.*
  - *Not destructible.*
- [ ] gold block
  - *Rewards the player with a bunch of points*
  - *Must not be a hit-to-win, just enough to perhaps turn the tide of a score.*

### Event blocks
These can handicap or reward the game/player for a fixed amount of seconds, somehow indicated on the screen.
- [ ] sticky-bar / greased-bar block
  - *Reduces or increases the speed of bar movement.*
- [ ] sticky-ball / greased-ball block
  - *Increases the speed of ball.*
- [ ] reverse-input block
  - *Reverses the input of a player, left goes right, right goes left.*
- [ ] drunk input block
  - *Causes a slight input delay.*
- [ ] resizing bar block
  - *Resizes the bar of a player. Could be either larger or smaller.*
- [ ] terraforming block
  - *Changes the shape of the player's bar - to either concave or convex shape.*
- [ ] wild block
  - *Causes the ball to randomly reverse or change movement.*
- [ ] gravity block
  - *Causes the ball to be affected by slight gravity on the last touched bar, causing it to curve.*
- [ ] rainbow block
  - *Causes the ball to change ownership to other player (random if multile).*
- [ ] lights-off block
  - *Turns the playing field dark, only illuminating the un-obstructed FOV of the ball.*
  - *Difficult to implement, requires shaddows*
- [ ] resizing ball block
  - *Changes the size of the ball, likely only increasing it will make a change, as the ball should be small enough as it is.*
- [ ] double points
  - *Scores are counted x2 for a short amount of time*
  - *Bonuses stack - hitting two of these blocks should give x4 points.*
- [ ] fire block
  - *Ball passes through the destructible blocks, not bouncing away.*
  - *Fixed amount of seconds, or untill next bar touches it?*
- [ ] tilted bar block
  - *Causes the bar to be slightly tilted in one direction, bouncing the ball in an odd angle*
- [ ] trembling bar block
  - *Causes the bar to be slightly rotating/vibrating, making it difficult to predict the bounce away*
- [ ] random block
  - *Performs a random event of other blocks, introduced somehow with a spinning wheel*

## Code structure, Objects & Features
- [ ] Scoring
  - *Scoring system - important to keep the balancing and fun in mind.*
  - *Highscore tracking.*
- [ ] Color themes
  - *Different color themes of neon-like variety*
- [ ] Main menu
  - *To select game mode/type, set options and such...*
- [ ] Background
  - *Some animation to break the monotony.*
- [ ] Levels
  - *Different types of level layouts, predetermined block layout.*
- [x] Meshes
  - *Should work towars meshes only being created once and repeated over the frame, instead of creating and drawing it every time.*
- [x] File configuration
  - *Ability to define levels and game configurations in a file path. Likely a json or yaml file.*
- [ ] Sound effects
  - *Hits, special blocks, actions/handicaps, countdowns...*
- [ ] Textures
  - *Special blocks are to be textured instead just being different color.*
- [ ] Mouse input
  - *Support for mouse control input.*
