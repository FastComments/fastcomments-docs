Istnieją cztery typy zdarzeń webhook agenta. Każde zdarzenie ma numeryczną wartość enum (używaną w payloadach) oraz kanoniczną nazwę łańcuchową (używaną w polu koperty `event` oraz w nagłówku HTTP `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Uruchomienie agenta kończy się statusem `SUCCESS`. |
| `trigger.failed` | 1 | Uruchomienie agenta kończy się statusem `ERROR`. |
| `approval.requested` | 2 | Zatwierdzenie jest umieszczone w stanie `PENDING`. |
| `approval.decided` | 3 | Zatwierdzenie przechodzi do `APPROVED`, `REJECTED` lub `EXECUTION_FAILED`. |

### `trigger.succeeded`

Wysyłane po pomyślnym zakończeniu uruchomienia agenta bez błędów. Pole `data` w payloadzie zawiera:

- `triggerId` - unikalny identyfikator uruchomienia.
- `triggerType` - [trigger reason enum](#triggers-overview), który uruchomił wykonanie.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokeny zużyte podczas tego uruchomienia.
- `wasDryRun` - true jeśli agent był w [dry-run mode](#dry-run-mode).
- `actions` - tablica rekordów `TenantAgentAction` (zob. [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - jeśli trigger je posiadał.

Jeśli uruchomienie nie wykonało żadnych akcji, tablica `actions` jest pusta - to pomyślne uruchomienie typu „agent zdecydował nic nie robić”, co jest przydatne do odnotowania.

### `trigger.failed`

Wysyłane, gdy uruchomienie zakończy się błędem. Ten sam kształt payloadu co w `trigger.succeeded`, z `status: 'ERROR'` i dodatkowym polem `errorMessage` opisującym, co poszło nie tak. Możliwe błędy obejmują awarie wywołań LLM, niepowodzenia dyspozycji narzędzi oraz wyczerpanie budżetu w trakcie uruchomienia.

`actions` mogą nadal zawierać wpisy dotyczące wywołań narzędzi, które zakończyły się przed wystąpieniem błędu.

### `approval.requested`

Wysyłane w momencie umieszczenia zatwierdzenia w stanie `PENDING`. Payload zawiera:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - argumenty narzędzia **przekazywane wprost** z wywołania LLM. Kształt jest specyficzny dla narzędzia i nie stanowi stabilnego publicznego kontraktu - schemat może się zmieniać w miarę dodawania nowych narzędzi.
- `createdAt`.
- `justification`, `confidence` - jeśli agent je dostarczył.
- `contextSnapshot` - kontekst komentarza / strony, którego dotyczy zatwierdzenie.

Przydatne do przesyłania oczekujących zatwierdzeń do kanału ChatOps: bot Slack subskrybujący `approval.requested` może opublikować akcję i uzasadnienie w kanale moderacji do szybkiego przeglądu.

### `approval.decided`

Wysyłane, gdy zatwierdzenie wychodzi ze stanu `PENDING`. Payload zawiera:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, lub `EXECUTION_FAILED`.
- `decidedBy` - identyfikator użytkownika moderatora, który podjął decyzję.
- `decidedAt` - kiedy podjęto decyzję.
- `executedAt` - jeśli APPROVED, kiedy platforma wykonała zatwierdzoną akcję.
- `executionResult` - jeśli APPROVED, łańcuch opisujący wynik wykonawcy.
- `contextSnapshot` - kontekst komentarza / strony.

To zdarzenie obejmuje wszystkie wyniki decyzji:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` ustawione, `executionResult` to komunikat o sukcesie.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` ustawione, `executionResult` opisuje niepowodzenie.
- **Rejected** -> `status: REJECTED`, `executedAt` jest null, `executionResult` jest null.

### Header

Każda dostawa zawiera nagłówek HTTP `X-FastComments-Agent-Event` z kanoniczną nazwą łańcuchową zdarzenia (`trigger.succeeded`, itd.). Przydatne, jeśli Twój endpoint to pojedynczy URL obsługujący wiele typów zdarzeń.

### See also

- [Webhook Payloads](#webhook-payloads) dla pełnych schematów payloadów dla poszczególnych zdarzeń.
- [Webhook Signing](#webhook-signing) dla schematu HMAC.
- [Webhook Retries](#webhook-retries) dla semantyki dostarczania.