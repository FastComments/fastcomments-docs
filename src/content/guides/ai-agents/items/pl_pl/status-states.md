Agent ma jeden z trzech statusów:

### Disabled

Agent jest wyłączony. Żadne wyzwalacze nie są przetwarzane i agent nie pojawia się w ścieżce przesyłania. Jego historia uruchomień, analityka i pamięć pozostają — jeśli ponownie go włączysz, dane historyczne nadal będą dostępne.

Use `Disabled` when:
- Chcesz wycofać agenta z rotacji bez jego usuwania.
- Agent zachowuje się nieprawidłowo i musisz go natychmiast zatrzymać, aby przeprowadzić dochodzenie.
- Sezonowo rotujesz agentami wchodzącymi i wychodzącymi (np. powitanie dostępne tylko w okresie świątecznym).

### Dry Run - domyślnie dla nowych agentów

Agent działa end-to-end - przetwarza wyzwalacze, wywołuje LLM, wybiera wywołania narzędzi, oblicza uzasadnienia i poziom pewności - ale **żadne rzeczywiste działanie nie jest wykonywane**. Każde uruchomienie jest zapisywane z odznaką **Dry Run** w [Historii uruchomień](#run-history).

Use `Dry Run` when:
- Nowy agent dopiero co został uruchomiony. Każdy szablon startowy trafia do trybu dry-run.
- Edytowałeś prompt lub zmieniłeś zestaw wyzwalaczy i chcesz zobaczyć, jak zmiana działa, zanim ją zatwierdzisz.
- Przeprowadzasz [testowe uruchomienie / powtórkę](#test-runs-replays) (powtórki wymuszają tryb dry-run niezależnie od statusu agenta).

Platforma pobiera tokeny za uruchomienia w trybie dry-run - wywołanie LLM nadal ma miejsce, pomijane są jedynie skutki uboczne. Limity budżetu dotyczą także dry-run. Zobacz [Przegląd budżetów](#budgets-overview).

### Enabled

Agent wykonuje rzeczywiste działania. Wywołania narzędzi są wykonywane - lub trafiają do kolejki oczekujących na [zatwierdzenie](#approval-workflow), jeśli wykonanie akcji wymaga zatwierdzenia.

Use `Enabled` after dry-run output looks correct.

### Switching status

Możesz przełączać się między dowolnymi dwoma statusami na formularzu edycji. Przełączenie z Dry Run na Enabled nie powoduje retroaktywnego ponownego wykonania działań z dry-run - te pozostają w historii dry-run. Nowe wyzwalacze od tego momentu będą działać na żywo.

Przełączenie z Enabled na Disabled w trakcie trwania uruchomienia **nie** przerywa bieżącego uruchomienia. Aktualnie wykonywany wyzwalacz kończy się (z tym, co już rozpoczął); następny wyzwalacz jest pomijany, ponieważ agent jest teraz Disabled.

### Status during billing problems

Jeżeli rozliczenia Twojego najemcy staną się nieprawidłowe, wszystkie agenty są de facto wstrzymane niezależnie od zapisanego statusu - wyzwalacze są odrzucane z powodem `BILLING_INVALID` aż do przywrócenia rozliczeń. Pole zapisanego statusu nie zostaje zmienione; dispatcher po prostu odmawia uruchomienia. Zobacz [Plany i uprawnienia](#plans-and-eligibility).