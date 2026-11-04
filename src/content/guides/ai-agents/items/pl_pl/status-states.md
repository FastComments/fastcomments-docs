Agent może mieć jeden z trzech statusów:

### Disabled (Wyłączony)

Agent jest wyłączony. Żadne triggery nie są przetwarzane, a agent nie pojawia się w ścieżce dispatch. Historia uruchomień, analityka i pamięć pozostają - jeśli ponownie go włączysz później, dane historyczne nadal tam będą.

Użyj `Disabled`, gdy:
- Chcesz wycofać agenta z rotacji bez jego utraty.
- Agent źle się zachowuje i musisz go natychmiast zatrzymać podczas dochodzenia.
- Sezonowo rotujesz agentów (np. powitalny agent działający tylko w okresie świątecznym).

### Dry Run - domyślny dla nowych agentów

Agent działa end-to-end - przetwarza triggery, wywołuje LLM, wybiera wywołania narzędzi, oblicza uzasadnienia i poziom pewności - ale **żadna prawdziwa akcja nie jest podejmowana**. Każde uruchomienie jest zapisywane z plakietką **Dry Run** w [Run History](#run-history).

Użyj `Dry Run`, gdy:
- Nowy agent jest świeżo po instalacji. Każdy szablon startowy ląduje w trybie dry-run.
- Edytowałeś prompt lub zmieniłeś zestaw triggerów i chcesz zobaczyć, jak zmiana się sprawdzi, zanim ją zatwierdzisz.
- Uruchamiasz [test run / replay](#test-runs-replays) (replaye wymuszają tryb dry-run niezależnie od statusu agenta).

Platforma nalicza tokeny za uruchomienia dry-run - wywołanie LLM nadal się odbywa, pomijane są tylko efekty uboczne. Limity budżetu dotyczą również dry-run. Zobacz [Budgets Overview](#budgets-overview).

### Enabled (Włączony)

Agent podejmuje prawdziwe akcje. Wywołania narzędzi są wykonywane - lub trafiają do kolejki [zatwierdzania](#approval-workflow), jeśli akcja jest chroniona.

Użyj `Enabled`, gdy wyniki dry-run wyglądają poprawnie.

### Zmiana statusu

Możesz przełączać się między dowolnymi dwoma statusami w formularzu edycji. Przełączenie z Dry Run na Enabled nie powoduje wstecznego ponownego wykonania akcji dry-run - pozostają one jako historia dry-run. Nowe triggery od tego momentu działają na żywo.

Przełączenie z Enabled na Disabled w trakcie uruchomienia **nie** przerywa uruchomienia w toku. Aktualnie wykonywany trigger kończy działanie (z tym, co już rozpoczął); kolejny trigger jest odrzucany, ponieważ agent jest teraz Disabled.

### Status podczas problemów z rozliczeniami

Jeśli rozliczenia Twojego tenanta staną się nieprawidłowe, wszyscy agenci są skutecznie wstrzymani niezależnie od zapisanego statusu - triggery są odrzucane z `BILLING_INVALID` do czasu przywrócenia rozliczeń. Zapisane pole statusu nie jest zmieniane; dispatcher po prostu odmawia uruchomienia. Zobacz [Plans and Eligibility](#plans-and-eligibility).