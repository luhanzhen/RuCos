

[//]: # (```mermaid)

[//]: # (graph LR)

[//]: # (    D&#40;[XConstraintSet]&#41; -.-> XConstraintType&#40;XConstraintType&#41;)

[//]: # (    XConstraintType -->  XExtension&#40;XExtension&#41; -.scope.-> Scope&#40;XVarVal&#41;)

[//]: # (    XConstraintType --> XAllDifferent&#40;XAllDifferent&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XAllDifferentExcept&#40;XAllDifferentExcept&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XInstantiation&#40;XInstantiation&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XAllEqual&#40;XAllEqual&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XOrdered&#40;XOrdered&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XRegular&#40;XRegular&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XMdd&#40;XMdd&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XIntention&#40;XIntention&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XGroup&#40;XGroup&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XSum&#40;XSum&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XMaximum&#40;XMaxMin&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XMinimum&#40;XMaxMin&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XElement&#40;XElement&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XSlide&#40;XSlide&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XCount&#40;XCount&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XNValues&#40;XNValues&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XCardinality&#40;XCardinality&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XChannel&#40;XChannel&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XCumulative&#40;XCumulative&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XNoOverlap&#40;XNoOverlap&#41;-.scope.-> Scope)

[//]: # (    XConstraintType -->XNoOverlapKDim&#40;XNoOverlap&#41;-.scope.-> Scope)

[//]: # (    XConstraintType --> XStretch&#40;XStretch&#41;-.scope.-> Scope)

[//]: # (    Scope -->IntVar&#40;IntVar is a variable&#41;)

[//]: # (    Scope -->IntVal&#40;IntVal is a value&#41;)

[//]: # (```)