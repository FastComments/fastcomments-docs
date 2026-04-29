Historia uruchomień to dziennik każdego uruchomionego wyzwalacza dla danego agenta. Dostępna ze strony listy agentów przez przycisk **Uruchomienia**, lub bezpośrednio pod `/auth/my-account/ai-agents/{agentId}/runs`.

### Co znajduje się na stronie

Paginowana tabela z jednym wierszem na uruchomienie:

| Kolumna | Znaczenie |
|---|---|
| Data | Kiedy wyzwalacz został uruchomiony (lub kiedy uruchomił się odroczony wyzwalacz). |
| Status | **Rozpoczęte**, **Powodzenie**, lub **Błąd**. Obok wyświetlana jest odznaka **Tryb testowy**, jeśli uruchomienie było w trybie dry-run. |
| Koszt | Koszt za uruchomienie w walucie Twojego najemcy. Puste dla uruchomień w toku (Rozpoczęte). |
| Akcje | Liczba wywołań narzędzi w trakcie uruchomienia. |
| Szczegóły | Przycisk **Zobacz**, który otwiera [Widok szczegółowy uruchomienia](#run-detail-view). |

### Znaczenie statusów

- **Rozpoczęte** - uruchomienie jest w toku, albo zakończyło się przed ukończeniem. Uruchomienie utknięte w "Rozpoczęte" przez nietypowo długi czas zazwyczaj oznacza przekroczenie limitu czasu wywołania LLM.
- **Błąd** - uruchomienie zostało zakończone, ale gdzieś wystąpił błąd — wywołanie LLM zwróciło błąd, wysłanie do narzędzia nie powiodło się itp. Widok szczegółowy zawiera konkretny błąd.
- **Powodzenie** - uruchomienie zakończyło się bez błędów. Agent mógł podjąć zero, jedną lub wiele akcji.

### Stan pusty

Gdy agent nie ma uruchomień, strona pokazuje: "Brak uruchomień dla tego agenta. Włączone uruchomienia pojawią się tutaj po uruchomieniu wyzwalacza; użyj testowego uruchomienia, aby podejrzeć, co ten agent zrobiłby względem wcześniejszych komentarzy."

Ten ostatni fragment jest celowy — [przebieg testowego uruchomienia](#test-runs-replays) to zalecany sposób na wypełnienie Historii uruchomień na świeżym agencie.

### Co nie znajduje się na stronie historii uruchomień

- **Live triggers that never dispatched** - wyzwalacz odrzucony z powodu budżetu, zakresu lub limitów szybkości nie pojawia się na tej stronie. Pojawiają się one na [stronie analityki](#analytics-page) pod "Triggers skipped".
- **Zatwierdzenia** - oczekujące zatwierdzenia dla akcji podjętych w tym uruchomieniu znajdują się w [skrzynce zatwierdzeń](#approval-workflow). Akcja wyświetla się w widoku szczegółowym uruchomienia jako **Oczekuje na zatwierdzenie**.

### Retencja

Poszczególne rekordy uruchomień są przechowywane przez 90 dni, po czym uruchomienie znika z historii. Koszty i liczniki wyzwalaczy są dalej agregowane w długoterminowych podsumowaniach analitycznych, więc [strona analityki](#analytics-page) nadal pokazuje historyczne sumy wykraczające poza ten okres.

### Odtworzenia

Uruchomienia wygenerowane przez odtwarzanie są domyślnie wykluczone z widoku uruchomień na żywo. To właśnie na stronie [Test Runs (Replays)](#test-runs-replays) możesz je zobaczyć.

### Filtrowanie między agentami

Tabela uruchomień jest per-agenta. Nie ma widoku obejmującego wielu agentów — [strona analityki](#analytics-page) jest podsumowaniem międzyagentowym. Jeśli musisz sprawdzić uruchomienia dla wielu agentów, zdarzenia Webhooks `trigger.succeeded` i `trigger.failed` to te, które powinieneś przekazać do własnego systemu.