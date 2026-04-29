Agent webhooks are HTTP callbacks fired by the platform when an agent run completes or an approval changes state. Use them to forward agent activity into your own systems - moderation dashboards, audit logs, Slack channels, escalation tools.

Configured under the **Webhooks** tab on the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### Co jest dostarczane

Cztery typy zdarzeń:

- **`trigger.succeeded`** - przebieg agenta zakończył się pomyślnie.
- **`trigger.failed`** - przebieg agenta zakończył się błędem.
- **`approval.requested`** - akcja została umieszczona w kolejce do zatwierdzenia przez człowieka.
- **`approval.decided`** - zatwierdzenie zostało zaakceptowane, odrzucone lub wykonanie zakończyło się niepowodzeniem.

Zobacz [Webhook Events](#webhook-events), aby sprawdzić, które zdarzenia wywoływane są kiedy, oraz [Webhook Payloads](#webhook-payloads) dla schematu każdego z nich.

### Co nie jest dostarczane

- **Webhooki dla poszczególnych akcji narzędzi.** Przebieg, który wywołuje `pin_comment`, nie generuje oddzielnego webhooka dla przypięcia — akcja jest zawarta w ładunku przebiegu `trigger.succeeded`. Jeśli chcesz dostarczania dla każdej akcji osobno, parsuj tablicę `actions` w ładunku triggera.
- **Odpadnięte triggery.** Trigger, który nie jest wysłany (przekroczony budżet, nieprawidłowy zakres) nie powoduje wygenerowania webhooka. Upadki są widoczne tylko na [Analytics page](#analytics-page).
- **Triggery wygenerowane przez replay.** Testowe przebiegi nie wysyłają webhooków. Zobacz [Test Runs (Replays)](#test-runs-replays).

### Konfiguracja

Dla każdego skonfigurowanego webhooka:

- **URL** - punkt końcowy HTTPS, na który wysyłany jest POST.
- **Domain** - domena komentarza, do której odnosi się ten webhook (twój tenant może obsługiwać wiele domen). `*` pasuje do wszystkich domen; `*.example.com` to glob; dokładna domena pasuje tylko do tej jednej.
- **Events** - które z czterech typów zdarzeń subskrybować.
- **Agents** - puste oznacza „wszystkie agenty”, albo lista konkretnych identyfikatorów agentów do filtrowania.
- **Method** - POST lub PUT (domyślnie POST).
- **No-retry status codes** - kody odpowiedzi HTTP, które powinny być traktowane jako błędy terminalne i nie podlegać ponownym próbom (np. 410 Gone, 422 Unprocessable). Zobacz [Webhook Retries](#webhook-retries).

Wiele webhooków może subskrybować to samo zdarzenie — każdy jest kolejkowany niezależnie i dostarczany pod własny URL.

### Dopasowanie według domeny

Dane zdarzenie jest dostarczane do **każdego webhooka, którego pole `domain` pasuje do domeny komentarza**. Dopasowywanie używa prostego globu:

- Dokładnie: `example.com` pasuje tylko do `example.com`.
- Symbol wieloznaczny: `*` pasuje do każdej domeny.
- Glob dla subdomen: `*.example.com` pasuje do `blog.example.com`, `forum.example.com`, ale nie do samego `example.com`.

Domena w ładunku to pierwszy nie-null wynik z: pola `domain` komentarza, URL sparsowanego względem konfiguracji domen twojego tenant'a, lub wyszukania `Page` po `urlId`.

### Filtrowanie według agenta

Pole **Agents** pozwala webhookowi subskrybować tylko wybrane agenty. Puste oznacza „wszystkie agenty”. Gdy pole nie jest puste, webhook uruchamia się tylko dla agentów znajdujących się na liście.

Jest to przydatne, gdy masz jeden webhook dla zdarzeń moderacji, a inny dla zdarzeń zaangażowania, oba kierujące do różnych systemów downstream.

### Wysłanie testowe

Interfejs konfiguracji webhooka ma przycisk **Wyślij test**, który wysyła fałszywy ładunek do URL, abyś mógł zweryfikować łączność, podpisywanie i kod odpowiedzi twojego punktu końcowego bez oczekiwania na rzeczywiste zdarzenie.

### Logi dostarczeń

Każde dostarczenie (i każda ponowna próba) trafia na stronę [Logi dostarczeń webhooków](#webhook-logs), dzięki czemu możesz zobaczyć, co stało się „na łączu”.

### Podpisywanie

Każdy webhook jest podpisywany za pomocą HMAC-SHA256 przy użyciu sekretu API twojego tenant'a. Zobacz [Podpisywanie webhooków](#webhook-signing).

### Wymagania

Webhooki agenta wymagają ważnego rozliczenia na tenant'cie. Używają tej samej infrastruktury podpisywania/sekretów co istniejące webhooki komentarzy — jeśli już zintegrowałeś webhooki komentarzy (zobacz [Poradnik webhooków](/guide-webhooks.html)), integracja webhooków agenta ma ten sam kształt z nowymi typami zdarzeń.

---