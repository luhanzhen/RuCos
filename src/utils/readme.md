# the class diagram about this mod is following:

```mermaid

classDiagram
    

    class Clone {
    <<Trait>>
    }

    class Display {
        <<Trait>>
    }
    

    SetTrait --* Display
    SetTrait --* Clone

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


   class SparseSetOfReference {
    <<Struct>>
    
    
   

}
    class LinkedSet {
        <<Struct>>
       

    }

    class SpareSetMultiLevel {
        <<Struct>>

    }


    class SpareSet {
        <<Struct>>
       
    }
    
    class SpareSetCounter {
        <<Struct>>

    }


    SpareSetCounter  --|>  SpareSet

    SpareSetMultiLevel  --|>  SpareSet
    
    LinkedSet  ..|>   SetTrait

    SpareSet  ..|>   SetTrait

    SparseSetOfReference  ..|>   SetTrait


    class Index {
        <<Trait>>
    }
    class IndexMut {
        <<Trait>>
    }

    SpareSet  ..|>   Index

    SparseSetOfReference  ..|>   IndexMut

    SparseSetOfReference  ..|>   Index

```