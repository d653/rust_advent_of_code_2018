#### Part 1:
- we can find the frequency of each character by using a `HashMap` in asymptotically optimal time
- then, we can retrieve the result by the help of some `fold()`

#### Part 2:  
Let W be the number of words, and L be the length of the words.  
It is easy to solve it in `O(W^2 * L)`: for each pair of words check if they satisfy the criteria.  
It is also not too hard to solve it in `O( W * L^2 )`: produce a hashset where, for each word, we put all possible strings obtained by removing a single character from the original word.  

The given solution runs in `O( W * L)` expected time. The idea is:
- for each word compute a hash of each possible new word obtained by removing a single character
- use a hashmap to map these hashes to the position of the removed character and a pointer to the original string

It is possible to produce each of these hashes in constant time, by using a rolling hash. 
