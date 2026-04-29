Gdy wyzwalacz uruchamia się dla agenta, ale **nie** skutkuje wywołaniem LLM, platforma rejestruje „pominięcie” z powodem. Pominięcia pojawiają się na stronie [Analytics page](#analytics-page) pod nagłówkiem "Triggers skipped (this month)".

### Pełna lista powodów pominięć

| Reason | What happened |
|---|---|
| `agentDaily` | Osiągnięto dzienny limit budżetu agenta. |
| `agentMonthly` | Osiągnięto miesięczny limit budżetu agenta. |
| `tenantDaily` | Osiągnięto dzienny limit budżetu tenanta. |
| `tenantMonthly` | Osiągnięto miesięczny limit budżetu tenanta. |
| `qps` | Osiągnięto limit przepustowości agenta na minutę (przesuwające się okno 60s). |
| `concurrency` | Maksymalna liczba jednoczesnych uruchomień agenta była już osiągnięta. |

### Czego nie ma na tej liście

Wyzwalacz, który nigdy nie trafia na ścieżkę dispatch, nie jest „pominięty” z powodem — po prostu nie jest wysyłany. Obejmuje to:

- Agent jest **Wyłączony**.
- Komentarz wyzwalający nie pasuje do [URL/locale scope](#scope-url-locale) agenta.
- Działanie wyzwalające zostało wykonane przez tego samego agenta (zapobieganie pętli).
- Tenant ma nieprawidłowe rozliczenia.
- Agent nie znajduje się w planie tenanta.

To są ciche pominięcia, a nie pominięcia z powodu (drops). Nie pojawiają się na wykresie pominięć w Analytics.

### Odczytywanie pominięć w Analytics

Strona [Analytics page](#analytics-page) pokazuje:

- **Triggers skipped (this month)** - zliczenia pogrupowane wg powodu pominięcia.
- **Agents at or near their cap** - rozbicie per-agenta, które agenty zbliżają się do limitu, wraz z liczbą pominiętych wyzwalaczy w bieżącym okresie.

### Co zrobić, gdy widzisz pominięcia

- **`agentDaily` / `agentMonthly`** - własny limit agenta jest zbyt niski. Zwiększ limit w formularzu edycji lub zawęź zakres agenta (URL/locale, węższe wyzwalacze).
- **`tenantDaily` / `tenantMonthly`** - limit na poziomie konta jest zbyt niski. Zwiększ go w ustawieniach rozliczeń tenant, albo rozłóż wydatki na mniejszą liczbę agentów.
- **`qps`** - ruch osiąga limit na minutę w przesuwającym się oknie. Często znak, że wiralny wątek rozsyła wyzwalacze szybciej, niż agent może je przetworzyć. Pola agenta `maxTriggersPerMinute` i `maxConcurrent` ograniczają to; ich zwiększenie podnosi przepustowość, ale też zwiększa koszty w skokach.
- **`concurrency`** - ta sama przyczyna co w `qps`, ale dotycząca liczby jednoczesnych zadań. Zwiększ `maxConcurrent`, jeśli potrzebujesz większej równoległości.

### Pominięcia kontra błędy

Pominięcie oznacza „wyzwalacz nigdy nie został uruchomiony”. **Błąd** oznacza „wyzwalacz został uruchomiony, ale wywołanie LLM lub dispatch narzędzia nie powiodło się”. Błędy są śledzone oddzielnie na stronie [Run History](#run-history) (status `Error`).

### Pominięcia mogą też zatrzymać replaye

Te same powody pominięcia zatrzymują trwające [test runs / replays](#test-runs-replays). Replay zatrzymuje się ze statusem Error i komunikatem wskazującym, który budżet został przekroczony (na przykład dzienny budżet agenta).

### Zapobieganie pętlom jest świadomie ciche

Nie ma powodu pominięcia typu „ten wyzwalacz pochodził od innego agenta i został pominięty, aby zapobiec pętli”. Logowanie tego zaśmiecałoby analitykę bez użytecznego sygnału — zgodnie z projektem, agent fan-out nigdy nie powinien prowadzić do eksplozji wyzwalaczy. Jeśli podejrzewasz, że pętla jest tłumiona tam, gdzie nie powinna być, sprawdź [Comment Logs](/guide-moderation.html#comment-logs) - `botId` w komentarzach napisanych przez bota jest tym, na czym opiera się sprawdzenie pętli.