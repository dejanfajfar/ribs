# Combatant movement

<pre>
X X X X X X
X X X X X X
X X X X X X
X X X X X X
X X X X X X
</pre>

Given the above battle field. 

<pre>
X X X X X X
X X X X X X
X X O X X X
X X X X X X
X X X X X X
</pre>

_O_ is the combatant and with a max movement distance of 2 he could reach all blocks marked with a _!_

<pre>
X X ! X X X
X ! ! ! X X
! ! O ! ! X
X ! ! ! X X
X X ! X X X
</pre>

Schematic above represents the movement range. 

## Step calculation

<pre>
4 3 2 3 4 5
3 2 1 2 3 4
2 1 X 1 2 3
3 2 1 2 3 4
4 3 2 3 4 5
</pre>




https://stackoverflow.com/questions/2311486/how-to-calculate-the-shortest-path-between-two-points-in-a-grid

Dijkstra's algorithm