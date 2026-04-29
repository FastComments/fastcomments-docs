Agent webhooks są ponawiane w razie niepowodzenia. Dostarczenie jest **fire-and-forget z perspektywy agenta** - nieudane dostarczenie nie blokuje wykonania agenta ani nie cofa żadnych działań - a kolejka i cron sterują ponownymi próbami asynchronicznie.

### Model kolejki

Każde zdarzenie jest kolejkowane **raz na pasujący webhook**. Więc jeśli masz trzy webhooki subskrybujące `trigger.succeeded` dla danego agenta + domeny, platforma umieszcza w kolejce trzy dostarczenia; każde jest dostarczane i ponawiane niezależnie. Błąd na jednym webhooku nigdy nie wpływa na pozostałe.

### Co jest ponawiane

Dostarczenie jest ponawiane, gdy:

- Żądanie HTTP **nie zostanie ukończone** (błąd DNS, połączenie odrzucone, timeout).
- Kod odpowiedzi HTTP to dowolny nie-2xx status, który nie znajduje się na skonfigurowanej liście **Kody statusów bez ponawiania**.

Dostarczenie **nie jest ponawiane**, gdy:

- Kod odpowiedzi to `2xx` (sukces).
- Kod odpowiedzi znajduje się na skonfigurowanej liście **Kody statusów bez ponawiania**. Domyślnie lista ta jest pusta - każdy nie-2xx jest ponawiany.

### Konfigurowanie kodów bez ponawiania

Formularz konfiguracji webhooka ma pole **Kody statusów bez ponawiania** (wartość wielokrotna). Typowe wpisy:

- `410` - Gone. Twój endpoint został trwale przeniesiony lub zasób zniknął. Ponawianie tylko marnuje przepustowość po obu stronach.
- `422` - Unprocessable Entity. Twój endpoint zrozumiał ładunek, ale uznał go za nieprawidłowy. Ponawianie z tym samym ładunkiem da ten sam wynik.
- `400` - Bad Request, w tym samym duchu.

Dodanie tutaj kodu oznacza: gdy endpoint zwróci ten kod, oznacz dostarczenie jako failed-terminal i przestań ponawiać.

### Harmonogram ponawiania

Proces tła uruchamia się co kilka sekund i przetwarza każde dostarczenie, którego czas następnej próby już minął.

Po każdej porażce czas następnej próby jest przesuwany do przodu zgodnie z **liniowym backoffem**: czas oczekiwania rośnie jako `60 seconds * attempt count` (więc próba 1 czeka 1 minutę, próba 2 czeka 2 minuty, i tak dalej).

Po 99 nieudanych próbach (lub 3 w lokalnym środowisku deweloperskim), dostarczenie jest porzucane i usuwane z kolejki. Wpisy w logach dostarczeń nadal są zachowywane i pozostają widoczne na stronie [Dzienniki dostarczania webhooków](#webhook-logs) aż do ich wygaśnięcia.

### Idempotencja po Twojej stronie

Ponieważ ponawiamy, Twój endpoint **musi być idempotentny**. Ten sam `triggerId` (lub `approvalId`) może przyjść więcej niż raz. Twój endpoint powinien:

- Użyć unikalnego klucza (`triggerId` dla zdarzeń trigger, `approvalId` dla zdarzeń approval) jako tokenu deduplikacji.
- Akceptować duplikaty dostarczeń bez problemu (zwrócić 200 przy drugiej próbie).

Nie-idempotentny endpoint w końcu przetworzy niektóre dostarczenia podwójnie, szczególnie podczas przejściowych awarii, gdy jedno żądanie timeoutuje i jest ponawiane po 30 sekundach, ale oryginalne żądanie faktycznie się powiodło.

### Kolejność

Dostarczenia **nie są ściśle uporządkowane**. `trigger.succeeded` i późniejsze `approval.requested` (z tego samego przebiegu) mogą przyjść w dowolnej kolejności, jeśli jedno zostanie ponowione, a drugie nie. Twój endpoint nie powinien zakładać przyczynowego porządku.

Jeśli potrzebujesz kolejności, użyj znaczników czasowych - `occurredAt` w kopercie, plus `createdAt` triggera/approvala w bloku danych - aby odtworzyć kolejność po swojej stronie.

### Czyszczenie

Dostarczenia są usuwane z kolejki natychmiast, gdy albo się powiodą, albo osiągną limit prób. Platforma nie przechowuje terminalnie nieudanych dostarczeń w samej kolejce; trwały zapis każdej próby znajduje się na stronie [Dzienniki dostarczania webhooków](#webhook-logs).

### Gdzie szukać, gdy ponowienia się nie powiodą

Strona [Dzienniki dostarczania webhooków](#webhook-logs) to miejsce, gdzie zobaczysz, dlaczego webhook nie działa. Typowe przyczyny:

- **Błąd rozwiązywania DNS** - URL jest nieprawidłowy lub domena przestała istnieć.
- **Błędy TLS** - certyfikat Twojego endpointu jest nieprawidłowy lub wygasł.
- **Połączenie odrzucone / timeout** - Twój endpoint jest niedostępny.
- **Odpowiedzi 5xx** - Twój endpoint działa, ale zwrócił błąd. Ciało odpowiedzi (przycięte) jest zapisywane.
- **Odpowiedzi 4xx** - Twój endpoint odrzucił ładunek. Jeśli to jest zamierzone, dodaj kod do **Kody statusów bez ponawiania**.

### Wstrzymanie niesprawnego webhooka

Jeśli webhook ciągle się nie udaje, najczystszym rozwiązaniem jest jego usunięcie (lub tymczasowe wyczyszczenie listy subskrypcji zdarzeń). Platforma nie wyłącza automatycznie zawodnych webhooków - będą one nadal ponawiane, aż dostarczenie zostanie porzucone.