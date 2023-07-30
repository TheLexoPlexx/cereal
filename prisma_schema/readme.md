### Überlegungen zum Schema:

Ein Seriennummernteil ist ein Teil mit Seriennummer, was zu einem Projekt gehört und eine Anlagenbezeichnung hat.
Ein Teil mit Seriennnummer kann entweder ein Bauteil sein, oder eine Baugruppe, eine Baugruppe ist eine Liste von Bauteilen. Anders herum: Ein Modell Bauteil kann entweder ein Liste "gruppe" haben mit mehr Bauteilen oder als Sackgasse ein definitives Bauteil sein.

Wenn aber jetzt ein Bauteil innerhalb einer Baugruppe gescannt wird... muss einfach nur in der Baugruppe aufwärts gegangen werden bis 

Eine Baugruppe hat eine Seriennummer und eine Liste von Seriennummerbauteilen.
Jede Seriennummernbaugruppe gehört zu einer Anlage mit Anlagenbezeichnung. Dabei ist die Konfiguration der Anlage (DFP/EFP, 1k-2k, ...) nicht genau betitelt, ergibt sich aber aus den Seriennummerbauteilen.

Jede Anlage gehört zu einem Projekt.

Jedes Bauteil mit Seriennummer, hat ein Herstellungsdatum und Zeit sowie ein Einbaudatum, so kann über ein neueres Datum abgelesen werden, ob etwas ersetzt wurde.

Wird ein Einbau wird von einem Monteur durchgeführt? Sodass nachvollziehbar ist, wer es gemacht hat? Sinnnvoller wäre wahrscheinlich die Verknüpfung wer zu der gegebenen Zeit beim Kunden vor Ort war und das Einbaudatum bekommt einen boolean "nachtrag" worüber dann Umbauten vom Kunden