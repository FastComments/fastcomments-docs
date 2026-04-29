Wywołuje agenta, gdy komentarz zostanie edytowany.

### Kontekst, który otrzymuje agent

- Komentarz w jego aktualnej (po edycji) formie.
- **poprzedni tekst komentarza** jako oddzielny odgrodzony blok (`PREVIOUS_TEXT`). To jest unikalne dla wyzwalacza edycji - agent może porównać stan przed i po.
- Opcjonalna historia wątku / użytkownika / strony zgodnie z konfiguracją.

### Ważne

- Wyzwalacz uruchamia się przy każdej pomyślnej edycji, włącznie z edycjami wprowadzonymi przez moderatorów w imieniu użytkownika.
- Agentom nie udostępniono narzędzia do edycji komentarzy; agenci w ogóle nie mogą edytować komentarzy.
- Poprzedni tekst komentarza jest umieszczony w odgrodzonym bloku jako dane nieufne. Systemowy prompt platformy przypomina modelowi, aby nie wykonywał instrukcji znajdujących się wewnątrz odgrodzonych bloków — ma to tu znaczenie, ponieważ złośliwy użytkownik mógłby edytować komentarz, aby wstawić ładunek typu "ignore your previous instructions" skierowany do dowolnego agenta obserwującego zdarzenia edycji.

### Typowe zastosowania

- **Wykrywanie wtórnie wprowadzonej treści** - użytkownik edytuje wcześniej czysty komentarz, aby wstawić spam po tym, jak moderator zajął się czymś innym.
- **Śledzenie drobnych edycji** - jeśli Twoja społeczność traktuje edycje jako oddzielne zdarzenia z powodów audytu.

### Uwaga dotycząca kosztów

Wyzwalacze edycji widzą dwie kopie tekstu komentarza (nową wersję w standardowym bloku COMMENT, starą wersję w bloku PREVIOUS_TEXT). Dla długich komentarzy oznacza to w przybliżeniu podwojenie kosztu tokenów dla wykonania w porównaniu z wyzwalaczem `COMMENT_ADD` — miej to na uwadze przy budżetowaniu.