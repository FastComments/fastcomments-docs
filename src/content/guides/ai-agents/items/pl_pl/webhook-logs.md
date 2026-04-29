Każdy webhook agenta ma własny dziennik dostarczania. Dostępny z [strony listy webhooków](https://fastcomments.com/auth/my-account/ai-agents/webhooks) za pomocą przycisku **Logi** w każdym wierszu webhooka.

### Co znajduje się na stronie

Pagowana tabela z jednym wierszem na próbę dostarczenia:

| Column | Meaning |
|---|---|
| When | Kiedy próba miała miejsce. |
| Event | Typ zdarzenia (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | Status dostarczenia. |
| StatusCode | Kod HTTP zwrócony przez Twój endpoint, gdy jest dostępny. |
| Description | Krótki opis wyniku. Dla nieudanych dostarczeń, gdy nie otrzymano odpowiedzi HTTP, oryginalny komunikat o błędzie jest przechowywany jako `{error: <message>}`. |

Strona obsługuje tylko paginację — nie ma filtrów po statusie, typie zdarzenia ani zakresie dat.

### Co możesz zrobić z poziomu logów

- **Zdecyduj, czy dany kod statusu powinien trafić do Kody statusów bez ponownych prób.** Jeśli widzisz, że Twój endpoint zwraca ten sam `4xx` w kółko, to silny sygnał, że chcesz dodać go do **Kody statusów bez ponownych prób**, aby platforma przestała podejmować ponowne próby.

### Informacje o niepowodzeniach

Gdy dostarczenie nie powiodło się bez odpowiedzi HTTP (błąd DNS, odmowa połączenia, timeout, błąd TLS itp.), surowy komunikat o błędzie jest zapisywany jako `{error: <message>}`. Platforma nie kategoryzuje tych błędów w nazwane grupy jak `TIMEOUT` czy `DNS_ERROR` — przeczytaj bezpośrednio komunikat o błędzie, aby zobaczyć, co się stało.

Dla odpowiedzi HTTP kolumna StatusCode pokazuje kod zwrócony przez Twój endpoint. Typowe przypadki:

- **Wszystkie próby: `401` lub `403`** - Twój endpoint odrzuca podpis. Sprawdź, czy obliczasz HMAC nad `${timestamp}.${body}` i używasz właściwego sekretu. Zobacz [Podpisywanie webhooków](#webhook-signing).
- **Wszystkie próby: `422`** - Twój endpoint uważa, że payload jest nieprawidłowy. Albo napraw endpoint, albo dodaj `422` do **Kody statusów bez ponownych prób**, jeśli odrzucenie jest spodziewane dla niektórych zdarzeń.

### Kontekst pojedynczego dostarczenia

Każdy wpis w logu zawiera:

- `webhookId` - która konfiguracja webhooka wygenerowała to dostarczenie.
- `agentId` - którego agenta dotyczy dostarczenie.
- `triggerId` lub `approvalId` - podstawowy rekord.
- `domain` - dopasowana domena.

Możesz użyć tych danych, aby skorelować nieudane dostarczenie z uruchomieniem, którego dotyczy w [Historia uruchomień](#run-history).

### Przechowywanie

Wpisy `AgentSyncLog` mają jednolity TTL 1 roku liczony od `createdAt`, niezależnie od wyniku — zarówno udane, jak i nieudane dostarczenia są przechowywane przez ten sam okres. Po upływie okresu retencji wpis z dziennika znika.

Jeśli potrzebujesz długoterminowego audytu, trwały wzorzec to: niech **sam endpoint** zapisuje dostarczenia, które otrzymuje. To oddziela Twój dziennik audytu od polityki retencji platformy.

### Wysyłka testowa

Przycisk **Wyślij testowo** w formularzu konfiguracji webhooka zapisuje fałszywe dostarczenie w tej samej tabeli logów, dzięki czemu możesz sprawdzić łączność end-to-end przed poleganiem na rzeczywistych zdarzeniach. Testowe dostarczenia są wyraźnie oznaczone w logu, aby nie zanieczyszczać statystyk błędów produkcyjnych.

### Zobacz także

- [Przegląd webhooków](#webhooks-overview).
- [Ponawianie webhooków](#webhook-retries) — o semantyce ponowień, która napędza te logi.
- [Podpisywanie webhooków](#webhook-signing) — jak weryfikować na swoim endpointzie.