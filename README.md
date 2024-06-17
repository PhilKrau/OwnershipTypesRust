# OwnershipTypesRust
Seminararbeit zum Thema Ownership Types in Rust

# Ownership in Rust
Rust nutzt das Prinzip der Ownership von Objekten zur Speicherverwaltung. Insbesondere um die Verwaltung des auf dem Heap angelegten Speichers. Ähnlich wie Sprachen wie C und C++ besitzt Rust keinen Garbage Collector, dass bedeutet angelegter Speicher muss manuell freigegeben werden. Historisch zeigte sich jedoch das die selbständige Speicherverwaltung zu einigen Problemen führen kann. So muss angelegter Speicher genau einmal freigegeben werden. Um dies in Rust zu garantieren wird das Prinzip der Ownership verwendet. Ein Pointer der auf ein Objekt zeigt und somit seine Speicheraddresse kennt ist der Owner dieses Objekts, sobald dieser Pointer out-of-scope geht wird der verwiesene Speicherbereich automatisch freigegeben. Sollte nun eine zweite Variable angelegt werden, die das selbe Objekt referenziert, so ist dieser Pointer der neue Besitzer des Objekts. Dies hat zur Folge, dass die zuerst Angelegte Varible nicht länger auf das Objekt zugreifen kann um ein doppeltes freigeben des gleichen Speicherbereichs ausschließen zu können.

# Beispiele
## Move Semantik
In der folgenden Grafik ist zu erkennen das Pointer s1 ungültig wird, sobald Pointer s2 auf das selbe Objekt verweist.
<img src="images/Ownership.svg" width = "300" height="300">

Quelle: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

### Simples Beispiel

![Move Semantik Code Beispiel](images/image_move_semantik.png)

Speicher im Heap besitzt immer einen sogenannten Besitzer, in der Regel ist dieser ein Pointer auf das entsprechende Objekt (vgl. Zeile 8). Sobald ein weiterer Pointer auf das Objekt zeigt ist dieser der neue Besitzer des Objekts und der ursprüngliche Pointer ist nicht mehr gültig (vgl. Zeile 12). Falls das Objekt kopiert werden soll so muss dies explizit angegeben werden (vgl. Zeile 21).

### Beispiel mit Methoden

![Move Semantik mit Methoden](images/image_move_methods.png)

Sollten solche Objekte an Methoden übergeben werden, so ist der neue Besitzer des Objekts ebenfalls die Methode (vgl. Zeile 11). Nur falls die Methode das Objekt beziehungsweise den Pointer auf das Objekt wieder zurückgibt, wird das Objekt nicht aus dem Speicher entfernt (vgl. Zeile 14).

## Borrow Semantik

<img src="images/Borrowing.svg" width = "400" height="300">

```
fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
} // s1 goes out of scope and the String object is deleted

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but since it has no ownership the object is not deleted
```

Quelle: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

Wie in dem Beispiel zu sehen ist verweist bei die Referenz bei der Borrow-Semantik lediglich auf den eigentlichen Besitzer des Objekts

### nicht veränderbare Referenz

![Borrow Semantik](images/image_borrow.png)

Mithilfe der Borrow Semantik ist es auch in Rust möglich mehere Pointer auf das selbe Objekt verweisen zu lassen. Dies ist wie im obigen Code Beispiel zu sehen sowohl mit neuen Variblen als auch bei der Übergabe an Methoden möglich.

### veränderbare Referenz
![Mutable Borrow Semantik](images/mut_borrow.png)

Um veränderbare Referenzen zu erzeugen wird das Schlüsselwort "mut" verwendet, welches kurz für mutable (deutsch: veränderbar) ist. Pro Owner kann jedoch immer nur eine einzige solche veränderbare Referenz existieren.

# Error Meldungen
## Zugriff auf ungültigen Pointer
![Error Message](images/image_error.png)

Der Rust Compiler erkennt selbständig, dass hier versucht wird auf einen Pointer zuzugreifen, der aufgrund der Move-Semantik nicht mehr gültig ist. Es wird angegeben, an welcher Stelle im Quellcode der Pointer ungültig wird, außerdem wird ein möglicher Lösungvorschlag zurückgegeben. 

## Doppelte Mutable Referenz
![Double mutable Error](images/double_mutable.png)

In Rust ist immer nur maximal eine veränderbare Referenz auf ein Objekt zugelassen, um sogenannte Race-Conditions zu vermeiden. Folglich führt ein Zugriff auf die Variable "r1" nachdem eine zweite mutable reference angelegt wurde zu einem Fehler.

# Die Grenzen des Rust compilers
## Unerreichbarer Code
Der Folgende Code kompiliert nicht, da der Rust-Compiler nicht selbständig erkennt, dass ein Teil des Quellcodes niemals ausgeführt wird. Der Rust-Compiler untersucht den Code lediglich Zeile für Zeile und meldet deshalb einen Speicherfehler, der zur Laufzeit jedoch nie auftreten wird.
![Unreachable Code](https://github.com/PhilKrau/OwnershipTypesRust/assets/72097023/6eef900a-ebea-42fc-8964-9065b515aa17)

## Stack Overflow durch zyklische Referenzen
Eine Möglichkeit einen Speicherfehler in Rust zu erzeugen, ist das verwenden zyklischer Referenzen. In dem Beispiel [reference_cycles.rs](Beispiele_Speicherfehler/reference_cycles.rs) ist Code zu finden der mithilfe zyklischer Referenzen einen Stack-Overflow zur Laufzeit verursacht. In dem Beispiel werden zwei Nodes einer linked-List erzeugt, die auf die jeweils andere verweisen. Gibt man sich nun den Nachfolger des ersten Elements aus, so verweisen die beiden Objekte solange gegenseitig aufeinander, bis der Stack voll ist und das Programm abstürzt.

## Speicher Manuell Freigeben 
Auch in Rust ist es möglich Speicherbereiche manuell freizugeben. Im Beispiel [manual_free.rs](Beispiele_Speicherfehler/manual_free.rs) ist Rust Code zu finden, bei dem versucht wird auf ein Element eines Vektors zuzugreifen, welches bereits manuell gelöscht wurde. Dieses Problem wird zur Compile-Zeit nicht vom Rust-Compiler erkannt, führt jedoch beim ausführen des Programms zum Absturz.


 
