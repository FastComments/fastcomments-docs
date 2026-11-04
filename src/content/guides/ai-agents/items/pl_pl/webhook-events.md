Istnieją cztery typy zdarzeń webhooków agenta. Każde zdarzenie ma numeryczną wartość enum (używaną w payloadach) oraz kanoniczną nazwę tekstową (używaną w polu `event` koperty oraz w nagłówku HTTP `X-FastComments-Agent-Event`).

| Nazwa zdarzenia | Enum | Występuje gdy |
|---|---|---|
| `trigger.succeeded` | 0 | Uruchomienie agenta kończy się statusem `SUCCESS`. |
| `trigger.failed` | 1 | Uruchomienie agenta kończy się statusem `ERROR`. |
| `approval.requested` | 2 | Zatwierdzenie jest kolejkowane w stanie `PENDING`. |
| `approval.decided` | 3 | Zatwierdzenie przechodzi do `APPROVED`, `REJECTED` lub `EXECUTION_FAILED`. |

### `trigger.succeeded`

Występuje po zakończeniu uruchomienia agenta bez błędu. Pole `data` payloadu zawiera:

- `triggerId` - unikalny identyfikator uruchomienia.
- `triggerType` - [enum powodu wyzwolenia](#triggers-overview), który rozpoczął uruchomienie.
- `status` - `SUCCESS` (ciąg znaków).
- `tokensUsed` - tokeny zużyte w tym uruchomieniu.
- `wasDryRun` - true, jeśli agent był w [trybie dry-run](#dry-run-mode).
- `actions` - tablica rekordów `TenantAgentAction` (zobacz [Payloady Webhooków](#webhook-payloads)).
- `commentId`, `url`, `urlId` - jeśli wyzwalacz je miał.

Jeśli uruchomienie nie wykonało żadnych akcji, tablica `actions` jest pusta - to udane uruchomienie typu "agent zdecydował, że nic nie zrobi", co jest przydatną informacją.

### `trigger.failed`

Występuje, gdy uruchomienie zwraca błąd. Ten sam kształt payloadu co `trigger.succeeded`, z `status: 'ERROR'` oraz dodatkowym polem `errorMessage` opisującym, co poszło nie tak. Możliwe błędy obejmują niepowodzenia wywołań LLM, niepowodzenia wywołań narzędzi oraz wyczerpanie budżetu w trakcie uruchomienia.

`actions` może nadal zawierać wpisy dla wywołań narzędzi zakończonych przed błędem.

### `approval.requested`

Występuje w momencie, gdy zatwierdzenie jest kolejkowane w stanie `PENDING`. Payload zawiera:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenty narzędzia **przekazane dosłownie** z wywołania LLM. Kształt jest specyficzny dla narzędzia i nie stanowi stabilnego publicznego kontraktu - schemat może się zmieniać wraz z dodawaniem nowych narzędzi.
- `createdAt`.
- `justification`, `confidence` - jeśli agent je dostarczył.
- `contextSnapshot` - kontekst komentarza / strony, którego dotyczy zatwierdzenie.

Przydatne do przekazywania oczekujących zatwierdzeń na kanał chat ops: bot Slack subskrybujący `approval.requested` może publikować akcję i uzasadnienie na kanale moderacyjnym w celu szybkiego przeglądu.

### `approval.decided`

Występuje, gdy zatwierdzenie opuszcza stan `PENDING`. Payload zawiera:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` lub `EXECUTION_FAILED`.
- `decidedBy` - identyfikator użytkownika moderatora, który podjął decyzję.
- `decidedAt` - kiedy podjął decyzję.
- `executedAt` - jeśli APPROVED, kiedy platforma wykonała zatwierdzoną akcję.
- `executionResult` - jeśli APPROVED, ciąg znaków opisujący wynik wykonania.
- `contextSnapshot` - kontekst komentarza / strony.

To zdarzenie obejmuje wszystkie możliwe wyniki decyzji:

- **Zatwierdzone + wykonane poprawnie** -> `status: APPROVED`, ustawione `executedAt`, `executionResult` to komunikat o sukcesie.
- **Zatwierdzone + wykonanie nie powiodło się** -> `status: EXECUTION_FAILED`, ustawione `executedAt`, `executionResult` opisuje niepowodzenie.
- **Odrzucone** -> `status: REJECTED`, `executedAt` jest null, `executionResult` jest null.

### Nagłówek

Każde dostarczenie zawiera nagłówek HTTP `X-FastComments-Agent-Event` z kanoniczną nazwą tekstową zdarzenia (`trigger.succeeded` itd.). Przydatne, jeśli Twój punkt końcowy to pojedynczy URL obsługujący wiele typów zdarzeń.

### Zobacz również

- [Payloady Webhooków](#webhook-payloads) dla pełnych schematów payloadów dla każdego zdarzenia.
- [Podpisywanie Webhooków](#webhook-signing) dla schematu HMAC.
- [Ponowne próby Webhooków](#webhook-retries) dla semantyki dostarczania.