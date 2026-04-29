**Dry Run** to tryb bezpieczeństwa, w którym zaczyna każdy nowy agent. Agent wykonuje cały proces end-to-end, z wyjątkiem części, w której wchodzi w interakcję z Twoją społecznością.

### Co działa w trybie Dry Run

- Wyzwalacze uruchamiają się normalnie.
- Składane są prompt agenta, [wytyczne społeczności](#community-guidelines) oraz [kontekst](#context-options).
- Wywoływane jest LLM.
- Model wybiera wywołania narzędzi i dostarcza uzasadnienia oraz oceny pewności.
- Uruchomienie jest zapisywane z odznaką **Dry Run**, dzięki czemu jest wyraźnie odróżnione od uruchomień na żywo.

### Co nie działa w trybie Dry Run

- Żaden komentarz nie jest publikowany, żaden głos nie jest oddawany, żaden komentarz nie jest przypinany/odpinany/zamykany/odblokowywany.
- Żaden komentarz nie jest oznaczany jako spam, zatwierdzany ani przeglądany.
- Żaden użytkownik nie jest zbanowany, ostrzegany ani nagradzany odznaką.
- Żaden e-mail nie jest wysyłany.
- Żadna pamięć nie jest zapisywana. (Tak — włącznie z pamięcią. Agenci w trybie dry-run nie mogą zatruwać wspólnej puli pamięci hipotetycznymi decyzjami.)
- Nie uruchamiają się webhooks dla akcji narzędzi. (Webhooks na poziomie wyzwalacza `trigger.succeeded` / `trigger.failed` wciąż się uruchamiają, a ładunek zawiera `wasDryRun: true`. Zobacz [Treści webhooków](#webhook-payloads).)

### Ile to kosztuje

Dry Run wykonuje **to samo wywołanie LLM**, co uruchomienie w trybie Enabled. Naliczane są tokeny, mają zastosowanie [limity budżetowe](#budgets-overview), a uruchomienia wliczają się do dziennych/miesięcznych limitów na agenta i na tenant.

Ten koszt to cena za wierne podglądanie działania. Tryb „pomiń wywołanie LLM” nie dałby żadnego sygnału o tym, jak agent by się zachował.

### Odczytywanie wyników dry-run

W [Historii uruchomień](#run-history) uruchomienia dry-run są oznaczone odznaką **Dry Run** w kolumnie statusu. Akcje w każdym uruchomieniu wyglądają identycznie jak akcje na żywo — ta sama nazwa narzędzia, te same argumenty, to samo uzasadnienie i ocena pewności — z tą różnicą, że żadna z nich nie miała miejsca.

[Strona analityczna](#analytics-page) rozbija uruchomienia „dry-run vs live” według miesięcy, dzięki czemu możesz zobaczyć, jaka część wydatków na tokeny poszła na obserwację.

### Przełączenie z Dry Run

Edytuj agenta i zmień **Status** na **Włączony**. Następne wyzwolenie uruchomi się na żywo.

Możesz też przełączyć w drugą stronę — z Enabled z powrotem na Dry Run — jeśli agent zacznie robić rzeczy, które Ci się nie podobają. Nie ma za to kary.

### Odtwarzania zawsze działają w trybie Dry Run

Funkcja [Uruchomienia testowe (Odtwarzania)](#test-runs-replays) uruchamia agenta przeciwko historycznym komentarzom **zawsze w trybie dry-run**, niezależnie od zapisanej konfiguracji agenta. Odtwarzania nie mogą podejmować prawdziwych działań wobec przeszłych komentarzy. To jest celowy zabieg — odtwarzanie to narzędzie do podglądu, a nie narzędzie do moderacji.