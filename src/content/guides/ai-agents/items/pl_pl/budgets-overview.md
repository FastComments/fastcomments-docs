---
Każdy agent ma limity wydatków. Platforma przestaje wysyłać zadania agenta, gdy osiągnięty zostanie limit, i wznawia je po rozpoczęciu nowego okresu.

### Dwa zakresy, dwa okresy

Są cztery limity łącznie — dwa zakresy (na agenta, na tenant) skorelowane z dwoma okresami (dziennym, miesięcznym).

| Scope | Period | Where you set it |
|---|---|---|
| Dzienny na agenta | dzień UTC | Formularz edycji agenta -> **Budżet** -> **Budżet dzienny** |
| Miesięczny na agenta | miesiąc kalendarzowy | Formularz edycji agenta -> **Budżet** -> **Budżet miesięczny** |
| Dzienny na tenant | dzień UTC | Pochodzi z planu (brak oddzielnego pola dla użytkownika) |
| Miesięczny na tenant | miesiąc kalendarzowy | Pochodzi z planu (brak oddzielnego pola dla użytkownika) |

Wyzwalacz jest uruchamiany tylko jeśli **wszystkie cztery limity** na to pozwalają. Pierwszy limit, który zostanie wyczerpany, powoduje odrzucenie wyzwalacza.

### Waluta

Budżety na agenta wprowadza się w walucie twojego konta.

### Co się dzieje, gdy limit zostanie osiągnięty

- Wyzwalacz jest zarejestrowany jako **odrzucony** z [powodem odrzucenia](#drop-reasons) takim jak `agentDaily` lub `tenantMonthly`.
- Liczba odrzuceń pojawia się na [Stronie analityki](#analytics-page) w sekcji "Triggers skipped (this month)".
- Nie odbywa się żadne wywołanie LLM; żadne tokeny nie są zużywane na sam odrzucony wyzwalacz.
- Status agenta pozostaje bez zmian — po prostu nie może się uruchamiać, dopóki okres się nie zresetuje.

### Reset okresu

- Limity **dziennie** resetują się o północy UTC.
- Limity **miesięczne** resetują się na początku każdego miesiąca kalendarzowego, UTC.

Niewykorzystany budżet nie jest przenoszony do następnego okresu.

### Twardy limit a miękkie ostrzeżenia

Limity są **twarde**. Nie ma trybu „przekroczyć o 10% z ostrzeżeniem”. Gdy limit zostanie osiągnięty, wysyłanie zostaje zatrzymane.

„Miękka” część to e-maile [Powiadomienia budżetowe](#budget-alerts) — otrzymujesz e-mail przy konfigurowalnych progach (domyślnie 80% i 100%), abyś mógł podnieść limit zanim ruch zacznie spadać.

### Gdzie sprawdzić aktualne zużycie

- [Strona analityki](#analytics-page) — zużycie budżetu na agenta i w skali tenant z oznaczeniami limitów.
- Sekcja **Stats** w formularzu edycji agenta.
- Widok listy (liczba oczekujących zatwierdzeń i ostatnie uruchomienia są na karcie agenta).

### Wybór budżetu

Kilka zasad orientacyjnych:

- **Nowy agent** — ustal budżet. Obserwuj [Historię uruchomień](#run-history) przez tydzień. Dostosuj na podstawie zaobserwowanego kosztu na uruchomienie × oczekiwanej liczby wyzwalaczy.
- **Agent o dużym natężeniu** (np. wyzwalacz 'new-comment' na ruchliwej stronie) — to limit dzienny złapie pętlę wymykającą się spod kontroli. Wybierz dzienny limit równy 2–3× oczekiwanego dziennego wydatku, aby normalnie ruchliwy dzień mieścił się poniżej niego.
- **Agent podsumowujący lub wykorzystujący dużo kontekstu** — koszt na uruchomienie jest wysoki. Ustaw niższy dzienny limit, by zapobiec temu, że zły dzień wyczerpie miesięczny budżet.

### Omijanie budżetu dla odtworzeń

[Testy / odtworzenia](#test-runs-replays) podlegają własnemu **twardemu** limitowi (ustawianemu w formularzu odtwarzania, oddzielnie od dziennych/miesięcznych limitów agenta) ORAZ limitom agenta i tenant. Ten, który zostanie osiągnięty jako pierwszy, zatrzymuje odtwarzanie.

### Zobacz też

- [Powiadomienia budżetowe](#budget-alerts) — o powiadomieniach e-mail.
- [Model kosztów](#cost-model) — jak platforma konwertuje tokeny na dolary.
- [Powody odrzucenia](#drop-reasons) — pełna lista powodów, dla których wyzwalacz nie uruchamia się.

---