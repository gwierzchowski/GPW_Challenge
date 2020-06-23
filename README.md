GPW Challenge
==============

This is the implementation of the problem solution in the recruitment for the position of Rust programmer (June 2020).
Exercise is closed, I do not predict any further development.

The problem
--------------
The digital maze with width X and length Y consists of zeros and ones, where zero is a wall
and one a way. Write an application in Rust that will find a way out of the maze with the fewest number of turns.
Starting point is a leftmost point in second row, and end point is rightmost point in second last row (at the bottom).

As the additional mode lines of input file should be converted from binary to decimal notation.

The solution
-------------
I tried to model the problem using a graph with weights of transitions (and possible cycles), so as to apply one of the known algorithms for searching the optimal path - e.g. the Dijkstra algorithm.

In this particular case:

- the nodes are all points declared as "open", i.e. `1` in input file
- the weight of transition between nodes is not set up front, but is dynamically calculated based on previous path (the last node)
  and is equal 0 if we do not change direction or 1 with direction change (the turn).

For the ease of explanation I will be using terms:

- I will use terms "way" and "path" interchangeably - for the same thing
- "the simplest way" for the path with least number of turns (i.e. the goal of exercise)
- "the rank of the way" - number of turns on the way
- "the rank of the node" - rank of the known way to given node from starting point

At the beginning of the procedure, the program puts all nodes in a container called `world`. This corresponds to the state "we do not know any path to any node".
Exception is the starting point, which is put into container `heaven` together with rank of simplest way (0 in this case) and the previous node on the path (itself in this case).
We create also the empty container: `purgatory`.

We begin from starting point. In each step there is one, current node which is already in `heaven` and we check all neighbor nodes:

- if neighbor is in `heaven`, then skip
- if neighbor is in `purgatory`, then we calculate its rank based on the way via current node (current node's rank maybe increased by one if there is a turn - based on the previous node to current one - which is remembered in `heaven`),
  and then we compare that with this neighbor known rank (being remembered in `purgatory`) - if it is smaller (i.e. better) we store it and store the current node as the previous one for the neighbor.
- in the else case (neighbor is in `world`), we calculate neighbor rank and previous node (as in last point) and move the node to `purgatory` with this information.

Therefore from the nodes in `purgatory` we select the one with smallest rank and:

- if there is no such (`purgatory` is empty), then exit
- if the minimal one is the end-point, then we move it to `heaven` and exit
- in else case we move minimal node to `heaven` together with its rank and previous node reference, set this node as current and go to loop begin (checking neighbors)

At the end we check if end-node is in `heaven`, if yes then we read its rank and present as solution (full path can be obtained using previous nodes information).
If end-node is not in `heaven`, then we declare that case have no solution (i.e. path from start to end does not exist).

Nodes move only in one direction: `world` -> `purgatory` -> `heaven`. In addition at each step we remove one node from `purgatory`. 
This means that procedure will not fall into infinite loop. Also the set neighbor's rank is >= current node rank, and as current we choose "weak minimum", 
this means that in each step until end, we will not set rank smaller then current one. In the same time every current node is in `heaven`, 
what means that those nodes have smallest possible rank - among them the end-node.

Estimated complexity ~ 4*(X*Y)^2.

Program
--------
The main program takes data in established format from `stdin` and prints result (number of turns) to `stdout`.
Program returns 0 if there is result, -1 if there is no result, -2 if data are incorrect, too large, or there was other error.
Error messages are printed to `stderr`. Exit thru 'panic' could only happen in case of some undiscovered logical error in program.
Initial validation of data should eliminate errors of overflow type (if I did not missed something).
The program run with option `--dbg` prints to `stdout` also the full solution path (list of nodes).
The program run with any other option, according to challenge instruction cause converting input lines from binary to decimal notation.

Algorithms
----------
There are coded 2 similar algorithms working as described above.
They differ in using the array to store nodes position and relations (`dijkstra_speed`) v.s. hash set (`dijkstra_mem`).
The idea behind the second implementation was to store only meaningful / passing (1) nodes, what as I count could reduce RAM memory consumption.
As it turned out it did not confirmed - I suppose because of relative big 1 to 0 rate in "sensible" data sets, and also because of spare allocation in set.
At end of day `dijkstra_speed` turned out to be minimally best algorithm.
The measured time for the data of size 2000 x 2000, is about 23s with memory consumption +- 50MB.

There are 2 API functions to obtain solution:
```rust
fn solve(&self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)>
```
oraz:
```rust
fn solve_and_drop(self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)>
```

The first one allows user to interactively (in the loop) work on the same loaded data set.
The last one allows for more aggressive memory optimization (e.g. dropping no longer required objects during calculation).

To do
-------------
The time for exercise was limited (few days), so there is still some room for potential improvements in the future.
Things that came to my mind are:

- Few thinks noted in the code as `TODO`.
- Code documentation (rustdoc)
- More unit and integration tests.
- Benchmark tests (`cargo bench`)
- More formal correctness proof
- Potentially refactor to limit `as` castings in many places.

License
--------
MIT-like. Derivative work is possible, but it must reference the source.

Author
--------
Grzegorz Wierzchowski
gwierzchowski@wp.pl


