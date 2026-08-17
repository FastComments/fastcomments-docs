Run History jest dziennikiem per‑agent każdego wyzwalacza, który został uruchomiony. Dostępny z listy agentów poprzez przycisk **Runs**, lub bezpośrednio pod adresem `/auth/my-account/ai-agents/{agentId}/runs`.

### Co znajduje się na stronie

Paginowana tabela z jednym wierszem na uruchomienie:

| Column | Meaning |
|---|---|
| Data | Kiedy wyzwalacz został uruchomiony (lub kiedy uruchomiono opóźniony wyzwalacz). |
| Status | **Started**, **Success**, lub **Error**. Etykieta **Dry Run** jest wyświetlana obok, jeśli uruchomienie było w trybie testowym. |
| Koszt | Koszt za uruchomienie w walucie Twojego najemcy. Puste dla uruchomień w toku (Started). |
| Akcje | Liczba wywołań narzędzi w ramach uruchomienia. |
| Szczegóły | Przycisk **View**, który otwiera [Run Detail View](#run-detail-view). |

### Znaczenia statusów

- **Started** – uruchomienie jest w toku lub zakończyło się przed ukończeniem. Uruchomienie utknęło w stanie „Started” przez wyjątkowo długi czas zazwyczaj oznacza przekroczenie limitu czasu wywołania LLM.  
- **Error** – uruchomienie zakończyło się, ale niepowodzeniem w pewnym miejscu – wywołanie LLM zwróciło błąd, nie udało się wysłać narzędzia itp. Widok szczegółów zawiera konkretny błąd.  
- **Success** – uruchomienie zakończyło się bez błędu. Agent mógł podjąć zero, jedną lub wiele akcji.

### Stan pusty

Kiedy agent nie ma żadnych uruchomień, strona wyświetla: **„Brak jeszcze uruchomień dla tego agenta. Włączone uruchomienia pojawią się tutaj po wyzwoleniu wyzwalacza; użyj Test run, aby zobaczyć, co ten agent zrobiłby w odniesieniu do wcześniejszych komentarzy.”**

Ten ostatni fragment jest zamierzony – [test run flow](#test-runs-replays) jest zalecaną metodą wypełnienia Historii uruchomień na nowym agencie.

### Co nie znajduje się na stronie historii uruchomień

- **Live triggers that never dispatched** – wyzwalacz odrzucony z powodu budżetu, zakresu lub limitu szybkości nie pojawia się na tej stronie. Są wyświetlane na [Analytics page](#analytics-page) pod „Triggers skipped”.  
- **Approvals** – oczekujące zatwierdzenia dla akcji podjętych w tym uruchomieniu znajdują się w [approvals inbox](#approval-workflow). Akcja pojawia się w widoku szczegółów uruchomienia jako **Pending approval**.

### Retencja

Indywidualne rekordy uruchomień są przechowywane przez 90 dni, po czym uruchomienie znika z historii. Koszt i liczby wyzwalaczy nadal są sumowane w długoterminowych podsumowaniach analitycznych, więc [Analytics page](#analytics-page) nadal pokazuje łączne wartości historyczne poza tym oknem.

### Replays

Uruchomienia wygenerowane przez Replay są domyślnie wykluczone z widoku live‑runs. Strona [Test Runs (Replays)](#test-runs-replays) to miejsce, gdzie je zobaczysz.

### Filtrowanie między agentami

Tabela uruchomień jest per‑agent. Nie ma widoku uruchomień między agentami – [Analytics page](#analytics-page) jest podsumowaniem międzyagentowym. Jeśli potrzebujesz przeglądać uruchomienia w wielu agentach, zdarzenia [Webhooks](#webhooks-overview) `trigger.succeeded` i `trigger.failed` to te, które należy przekazać do własnego systemu.