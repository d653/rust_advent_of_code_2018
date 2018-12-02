The first part is trivial. 

Some notes about the second part:
- `.cycle()` allows to chain an iterator with itself infinitely often
- we can easily find the first duplicate by exploiting the fact that `HashSet::insert` returns `false` if the element is already present
