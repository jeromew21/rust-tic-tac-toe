# Tic Tac Toe
unbeatable AI and command line interface

learned a bit about AI algorithms. Originally had the minimax value a win/loss in the future the same as an immediate one, so it would rate all moves the same --- effectively rendering it useless. Fixed this by multplying the heuristic of an immediate win/loss by the current depth --- i.e. a win on depth 3 is 3 times better than than a win on depth 1, since the AI always looks 3 moves ahead. 
