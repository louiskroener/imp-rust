# imp-rust
basiert auf https://sulzmann.github.io/ModelBasedSW/imp.html#(1)
## Imp Rust

## Ziele

Ziel ist es, das vorhandene Go Projekt Imp in Rust zu implementieren.
Dabei sollen verschiedene Ansätze ausprobiert werden, dass in Rust zu realisieren.

## Ansätze

### "Go Model"

Beim "Go Model" geht es darum die Interfaces und Overloading aus Go in Rust zu übersetzten.  
In der Go Implementierung wird ein Interface als abstrakter Datentyp genutzt.
```
type Exp interface {
    pretty() string
    eval(s ValState) Val
    infer(t TyState) Type
}
```

Dieses Interface kann dann auf Typen implementiert werden
```
type Num int
type Mult [2]Exp
```

```

func (x Num) eval(s ValState) Val {
    return mkInt((int)(x))
}

func (e Mult) pretty() string {

    var x string
    x = "("
    x += e[0].pretty()
    x += "*"
    x += e[1].pretty()
    x += ")"

    return x
}
```

Das Interface kann anschließend als (abstrakter) Typ genutzt werden.  
Hier ein Beispiel aus den Hilfsfunktionen:

```
func and(x, y Exp) Exp {
    return (And)([2]Exp{x, y})
}
```

Das Interface kann mit Hilfe von  `Traits`[^1] implementiert werden
```
pub trait Exp {
    fn pretty(&self) -> String;
    fn eval(&self, s: &mut ValState) -> Val;
    fn infer(&self, t: &mut TyState) -> Type;
}
```
Die Structs und Typen funktionieren in Rust ähnlich.
```
pub type Num = i32;
pub struct Mult {
    pub exp: [Box<dyn Exp>; 2],
}
```
In Rust wird nun das `Trait` nun komplett für ein Typ implementiert
```
impl Exp for Mult {
    fn pretty(&self) -> String {
        .. code
    }
    fn eval(&self, s: &mut ValState) -> Val {
        .. code
    }
    fn infer(&self, t: &mut TyState) -> Type {
        .. code
    }
}
```
In Rust kann das Trait nicht direkt als Typ genutzt werden. In Rust muss noch das Keyword `dyn`[^2] genutzt werden
```
dyn Exp
```
Außerdem müssen wir Rust noch mitteilen, dass wir die Objekte auf dem Heap speichern wollen, da die Datenstrukturen rekursiv sind. Dazu nutzen wir `Box`[^3]
```
Box<dyn Exp>
```
Zusammengesetzt in der Hilfsfunktion: 
```
pub fn mult(x: Box<dyn Exp>, y: Box<dyn Exp>) -> Box<dyn Exp> {
    Box::new(Mult { exp: [x, y] })
}
```


### Generics
Die Generics Lösung ist gleich wie die Go Model Lösung strukturiert.
Am `Trait` ändert sich nichts, aber anstatt einen abstrakten Datentypen zu nutzen, werden Generics genutzt. 
```

pub struct Mult<T1: Exp, T2: Exp> {
    pub left: Box<T1>,
    pub right: Box<T2>,
}

impl<T1: Exp, T2: Exp> Exp for Mult<T1, T2> {
    fn pretty(&self) -> String {
       .. code
    }
    fn eval(&self, s: &mut ValState) -> Val {
       .. code
    }
    fn infer(&self, t: &mut TyState) -> Type {
        .. code
    }
}
pub fn mult<T1: Exp, T2: Exp>(left: Box<T1>, right: Box<T2>) -> Box<Mult<T1, T2>> {
    Box::new(Mult { left, right })
}
```

Zu beachten ist, dass hier das Struct selbst als Rückgabewert definiert, nicht wie im Go Model der Abstrakte Datentyp. Eventuell gibt es hier noch eine Lösung ein Generic zurückzugeben
### Enum Model

Das Enum Model kopiert nun nicht mehr das Go Model.
Anstatt des abstrakten Datentyps wird ein `Enum`[^4]genutzt.
```
pub enum Exp {
    Num { val: i32 },
    Mult { left: Box<Exp>, right: Box<Exp> },
    
}

```
`Box` wird weiterhin als Smart Pointer genutzt.  
Die Funktionen werden nun an das Enum gebunden. Mit 
`match`[^5] kann zwischen den Varianten des Enums unterschieden werden.
 
```
impl Exp {
    fn eval(&self, s: &mut ValState) -> Val {
        match &self {
                Exp::Num { val } => {
                        .. code
                },
                Exp::Mult { left, right } => {
                        ..code 
                }
        }
        .. 
```
Ein Problem bist, dass eine Enum Variante nicht direkt als Typ genutzt werden kann. Man kann es mit Structs umgehen, wie in diesem [Beispiel](https://stackoverflow.com/questions/29088633/grouping-structs-with-enums).  
Aber dadurch verlieren wir wieder unsere Enum Abstraktion und müssten wieder Methoden für die Structs definieren.  
Stattdessen kann auch das Enum als Rückgabetyp / Parameter nutzen, und jeweils eine Überprüfung auf die Variante einbauen. Dazu kann man wieder `match` oder `if let`[^6] nutzen
Die Enum Variante wirkt wie eine Rust idiomatische Implementierung.



### Links
https://doc.rust-lang.org/book/ch10-02-traits.html  
https://doc.rust-lang.org/std/keyword.dyn.html  
https://doc.rust-lang.org/book/ch15-01-box.html  
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html  
https://doc.rust-lang.org/book/ch06-02-match.html  
https://doc.rust-lang.org/book/ch06-03-if-let.html  


[^1]: https://doc.rust-lang.org/book/ch10-02-traits.html
[^2]: https://doc.rust-lang.org/std/keyword.dyn.html
[^3]: https://doc.rust-lang.org/book/ch15-01-box.html
[^4]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[^5]: https://doc.rust-lang.org/book/ch06-02-match.html
[^6]: https://doc.rust-lang.org/book/ch06-03-if-let.html