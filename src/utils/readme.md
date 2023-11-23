# the class diagram about this mod is following:

```mermaid

classDiagram
direction LR

    class SetTrait {
        <<Trait>>
        + fn add(&mut self, ele: T);

        + fn delete(&mut self, ele: T);

        +  fn contains(&self, ele: usize) -> bool;

        + fn size(&self) -> usize;

        +  fn clear(&mut self);

        + fn is_empty(&self) -> bool;

        +  fn max_size(&self) -> usize;
    }


   
   


    class LinkedSet {
        <<Struct>>
         size: usize,
         first: usize,
         last: usize,
         last_removed: usize,
         nb_levels: usize,
         limits: Box<Vec<usize>>,
         prev: Box<Vec<usize>>,
         removed_levels: Box<Vec<usize>>,
         prev_removed: Box<Vec<usize>>,
         next: Box<Vec<usize>>,

        pub fn record_limit(&mut self, level: usize)
        + fn reduce_to(&mut self, ele: usize, level: usize) -> usize
        pub fn next(&self, ele: usize) -> Option<usize>
        
        
    }

    class EmptyDomainException {
        <<Struct>>
        +  fn new(&str)->Self
    }


    EmptyDomainException  ..|>  SetTrait
    LinkedSet  ..|>   SetTrait

```