# Test Cases

This file contains a bulleted list of functionality. In the absence of any automated tests, these should be verified manually before releases are published.

### Networking
* Multiple clients should be able to connect simultaneously
* All maps should download from server successfully
* Client/server should work across the internet and not just from the same local machine
* Client should be able to create new character
* Client should be able to log in to existing character
* Client should be kicked if trying to log in to existing character currently online
* Client should gracefully log out when using ctrl-q

### Server Initialisation
* Input validation on server address:port
* Server should load existing world backup
* Server should initialise new world when existing world backup doesn't exist

### Client General
* Input validation on player name, server address:port
* Client window should be able to be dynamically resized
* Client sidebar should display correct values
* Client top-bar should display correct player position
* System messages should display in correct order with correct colours
* Movement and combat of players/monsters should not feel laggy

### Collisions
* Player should be able to collide with map tiles correctly
* Player should be able to collide with other players/monsters and initiate combat
* Monsters should collide with each other and not initiate combat
* Monsters should collide with map tiles correctly
* Player and monsters should be able to walk around the entire map edge against the boundary without server crashing
* Entities should not be able to occupy the same tile except on respawning, player login, or new player creation

### Players General
* Player look command should show all types of tile and other players/monsters
* Player drop command when nothing held should return message
* Player pickup command when nothing to pickup should return message
* Player pickup when item on floor should pick up item and remove it from world for all players
* Player drop command should allow selection and drop correct item to world for all players
* Player drop command selection should handle pagination when more than 10 items carried
* Player should level up when meeting the exp_next threshold
* Player stat choice should be available on levelling up
* Player stat choice should only be available for stats below 100

### Monsters General
* Monsters should move towards player when nearby
* Monsters should stop following player if player dies or logged out
* Monsters should initiate combat with player if adjacent
* Monsters should wander randomly when not moving towards player

### Combat
* Killed monster should be removed from world
* Killed player should respawn back at default location
* Killed monster should drop what it is carrying
* Killed player should drop nothing
* Player should gain exp and gold when monster is killed
* Player should gain gold only when other player killed
* Monster should gain nothing from killing players
* Health regen should only work when out of combat
* Combat should not occur if player moves away from target before server tick
* Monsters should change target if attacked
* Monsters and players should not be able to attack after they have died