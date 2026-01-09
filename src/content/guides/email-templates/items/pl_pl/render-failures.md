Ponieważ szablony e-mail obsługują zmienne, i logikę, możliwe jest stworzenie szablonów
które nie renderują się, lub czasami zawodzą podczas renderowania.

To może być bardzo frustrujące do zdiagnozowania i debugowania, szczególnie jeśli jest to problem przerywany, lub
jeśli występuje tylko wtedy, gdy dane wyglądają w określony sposób.

Aby pomóc, FastComments Email Templates ma kilka funkcji:

1. Jeśli szablon nie zajdzie w podglądzie, nie można go zapisać. Zostanie wyświetlony komunikat o błędzie.
2. Błędy renderowania szablonów są śledzone i raportowane w panelu administracyjnym.

Drugi punkt opisuje błędy renderowania, które występują w produkcji. Czyli tworzymy szablon, który w podglądzie
wygląda poprawnie - ale później z jakiegoś powodu przestaje działać. Na przykład, jeśli mamy to w naszym szablonie:

    <% if (comment.commenterEmail.includes('test') { %>

To może czasami zawodzić, jeśli mamy włączone anonimowe komentowanie, ponieważ e-mail nie zawsze będzie
dostępny. Jak więc się o tym dowiedzieć?

Odpowiedź jest taka, że błędy są ujawniane w dwóch miejscach. Po pierwsze, sama lista szablonów
pokazuje przy każdym szablonie liczbę błędów renderowania.

Następnie, podczas przeglądania szablonu, możemy zobaczyć liczbę, dla każdego błędu, ile razy szablon
nie udało się wyrenderować.

Przycisk resetowania znajduje się obok każdego błędu i jego licznika, abyśmy mogli zresetować licznik
po rozwiązaniu problemu.