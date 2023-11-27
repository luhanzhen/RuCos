

```mermaid
classDiagram
direction BT

    class DomainTrait {
        <<Trait>>
       
    }
    
    
    class DomainRange {
    
        }
    
    class DomainValues {
    
    }


    DomainValues  ..>  DomainTrait
    DomainRange  ..>  DomainTrait

```