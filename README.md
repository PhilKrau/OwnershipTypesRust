# OwnershipTypesRust
Seminararbeit zum Thema Ownership Types in Rust

# Ownership in Rust
Rust nutzt das Prinzip der Ownership von Objekten zur Speicherverwaltung. Insbesondere um die Verwaltung des auf dem Heap angelegten Speichers. Ähnlich wie Sprachen wie C und C++ besitzt Rust keinen Garbage Collector, dass bedeutet angelegter Speicher muss manuell freigegeben werden. Historisch zeigte sich jedoch das die selbständige Speicherverwaltung zu einigen Problemen führen kann. So muss angelegter Speicher genau einmal freigegeben werden. Um dies in Rust zu garantieren wird das Prinzip der Ownership verwendet. Ein Pointer der auf ein Objekt zeigt und somit seine Speicheraddresse kennt ist der Owner dieses Objekts, sobald dieser Pointer out-of-scope geht wird der verwiesene Speicherbereich automatisch freigegeben. Sollte nun eine zweite Variable angelegt werden, die das selbe Objekt referenziert, so ist dieser Pointer der neue Besitzer des Objekts. Dies hat zur Folge, dass die zuerst Angelegte Varible nicht länger auf das Objekt zugreifen kann um ein doppeltes freigeben des gleichen Speicherbereichs ausschließen zu können.

# Beispiele
## Move Semantik
In der folgenden Grafik ist zu erkennen das Pointer s1 ungültig wird, sobald Pointer s2 auf das selbe Objekt verweist.
<img src="images/Ownership.svg" width = "300" height="300">

Quelle: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

Nach dem ein Wert ver

# Error Meldungen
<img src="images/image.png">
Der Rust Compiler erkennt selbständig, dass hier versucht wird auf einen Pointer zuzugreifen, der aufgrund der Move-Semantik nicht mehr gültig ist. Es wird angegeben, an welcher Stelle im Quellcode der Pointer ungültig wird, außerdem wird ein möglicher Lösungvorschlag zurückgegeben. 

# Aufgaben
- Warum werden primitive Typen kopiert ? --> feste größe im Speicher
- Lesezugriffe für nicht- owner --> Borrow Semantik (static reference & mutable reference)
- Allgemein Vergleich Move- Copy- Borrow-Semantik
- Beispiel in mehrere kleine (Anwendungs-)beispiele unterteilen

- Vergleich C++ 
    - Code zeigen
    - C++ Runtime <--> Rust Compile-Time

- Error Meldungen vorstellen
    - Typfehlermeldungen (wegen Speicherfehler)
    - Evtl noch Tool(s) suchen die bei der Lösung dieser Probleme helfen

- Siehe Readme Repo Prof. Sulzmann
 