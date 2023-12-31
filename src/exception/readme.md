
# the class diagram about the *ExceptionFactory*

```mermaid
classDiagram
direction LR

    class ExceptionFactory {
        <<Service>>
        }

    class  ExceptionType {
        <<Enumeration>>
        + EmptyDomainExceptionType,
        + InvalidVariableExceptionType,
        + UnsatisfiedConstraintException,
    }

    ExceptionFactory --* ExceptionType
```

# the class diagram about the *ExceptionTrait*



```mermaid
classDiagram
direction LR

    class ExceptionTrait {
        <<Trait>>
        
        fn message(&self)  &str;

        fn exception_level(&self)  &ExceptionLevel;

        fn is_fatal(&self)  bool;
    }


    class ExceptionLevel {
        <<Enumeration>>
        +  Ignorable,
        +  Minor,
        +  Major,
        +  Fatal,
    }

    ExceptionTrait ..o ExceptionLevel


    class InvalidVariableException {
        <<Struct>>
    }

    class EmptyDomainException {
        <<Struct>>
       
    }

    class UnsatisfiedConstraintException {
        <<Struct>>
      
    }
    UnsatisfiedConstraintException  ..|>  ExceptionTrait
    EmptyDomainException  ..|>  ExceptionTrait
    InvalidVariableException  ..|>   ExceptionTrait

```