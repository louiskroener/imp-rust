# imp-rust
basiert auf https://sulzmann.github.io/ModelBasedSW/imp.html#(1)

## Ziele

Ziel ist es, das vorhandene Go Projekt Imp in Rust zu implementieren.
Dabei sollen verschiedene Ansätze ausprobiert werden, das in Rust zu überstetzen.

## Ansätze

### "Go Model"
o
Die Go Interfaces in Rust übersetzten. Das Rust selbst direkt keine Interfaces bietet, mit dyy box model arbeiten
´´´ dyn<Box<>> 

### Enum Model
Problem: Enum Varianten sind kein Typ => Es können keine Expliziten Varianten als rückgabe oder argument typ genutzt werden.
Umgehen: Mit Extra Structs. Wie in Go Model werden die Structs dann in die Enum Typen gegeben und es wird mit den Structs gearbeiter. 
        Diese Lösung ist aber nicht besonders hilfreich, da wieder mit Structs gearbeitet wird

Siehe: https://stackoverflow.com/questions/29088633/grouping-structs-with-enums


### Generics
fällt auch unter go model
Eigentlich wie die dyn Lösung, bildet die Go Lösung aber nicht direkt ab

### weitere ideen

## Rust Learnings:
Halbes Learning, einfach auf Types achten (s.get(name)) 
https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str

 https://stackoverflow.com/questions/29088633/grouping-structs-with-enu




