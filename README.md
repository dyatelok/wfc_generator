## wfc_generator

# Wave function collapse algorithm

It's a wave function collapse algorithm implementation using rust. There ate 3 different input options. To run use `cargo run --bin wfc-generator`

1. You can use manual input and specify, what kinds of tiles can be near in different directions to each other. You should change the `data/tiles-data.txt` file. The format is:
   
   tile id
   
   path_to_file rotation_of_texture(number from 0 to 3)
   
   8 lines with possible neibors in different directions un order:  (-1,-1),  (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1).

2. You can use simple-input and change `data/simple-input.txt`. It uses gates system. You have different connections between your tiles. If tiles can connect with each other you should write the same digit (gate type) for them to connect. Tiles are rotated, so you should specify each texture onle once with gates in format:
   path_to_texture gates in directions up,left,right,down.

3. You also can use graphical editor, to make a sample and then turn them into the actual data. Use `cargo run --bin map-to-data`
   
   Controls are:
   
   arrows -> cursor moving
   
   V -> on/off selection mode, makes action to the whole selected region
   
   Y -> copy color under the cursor
   
   P-> paste color
   
   N -> next color in palet
   
   L -> load from file (`data/map.txt`)
   
   S -> write to file
   
   T -> transform into the data for generation
   
   Method uses N=3 neiborhood.
