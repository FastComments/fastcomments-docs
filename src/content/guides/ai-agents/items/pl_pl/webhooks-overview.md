Webhooki agentów to wywołania zwrotne HTTP wyzwalane przez platformę, gdy przebieg agenta zostaje zakończony lub gdy zmienia się stan zatwierdzenia. Użyj ich, aby przekazywać aktywność agentów do własnych systemów - paneli moderacji, dzienników audytu, kanałów Slack, narzędzi eskalacji.

Konfigurowane w zakładce **Webhooks** na [stronie AI Agents](https://fastcomments.com/auth/my-account/ai-agents).

### Co jest dostarczane

Cztery typy zdarzeń:

- **`trigger.succeeded`** - przebieg agenta zakończył się pomyślnie.
- **`trigger.failed`** - przebieg agenta zakończył się błędem.
- **`approval.requested`** - akcja została zakolejkowana do zatwierdzenia przez człowieka.
- **`approval.decided`** - zatwierdzenie zostało zaakceptowane, odrzucone lub wykonanie nie powiodło się.

Zobacz [Webhook Events](#webhook-events), aby dowiedzieć się, które zdarzenia kiedy są wyzwalane, oraz [Webhook Payloads](#webhook-payloads) dla schematu każdego z nich.

### Co nie jest dostarczane

- **Webhooki dla poszczególnych akcji narzędzi.** Przebieg, który wywołuje `pin_comment`, nie wyzwala osobnego webhooka dla przypięcia - akcja jest zawarta w ładunku `trigger.succeeded` przebiegu. Jeśli chcesz dostarczania per-akcja, parsuj tablicę `actions` w ładunku triggera.
- **Odrzucone triggery.** Trigger, który nie został wysłany (przekroczony budżet, niewłaściwy zakres), nie wyzwala webhooka. Odrzucenia są widoczne tylko na [stronie Analytics](#analytics-page).
- **Triggery wygenerowane przez replay.** Przebiegi testowe nie wyzwalają webhooków. Zobacz [Test Runs (Replays)](#test-runs-replays).

### Konfiguracja

Dla każdego webhooka ustawiasz:

- **URL** - endpoint HTTPS, na który wysyłane jest żądanie POST.
- **Domain** - domena komentarzy, której dotyczy ten webhook (Twój tenant może obsługiwać wiele domen). `*` pasuje do wszystkich domen; `*.example.com` to glob; dokładna domena pasuje tylko do niej samej.
- **Events** - które z czterech typów zdarzeń subskrybować.
- **Agents** - puste oznacza "wszyscy agenci", lub lista konkretnych ID agentów do filtrowania.
- **Method** - POST lub PUT (domyślnie POST).
- **No-retry status codes** - kody odpowiedzi HTTP, które powinny być traktowane jako błędy końcowe, bez ponawiania (np. 410 Gone, 422 Unprocessable). Zobacz [Webhook Retries](#webhook-retries).

Wiele webhooków może subskrybować to samo zdarzenie - każdy z nich jest kolejkowany niezależnie i dostarczany na własny URL.

### Dopasowanie per-domena

Dane zdarzenie jest dostarczane do **każdego webhooka, którego pole `domain` pasuje do domeny komentarza**. Dopasowanie używa prostego globu:

- Dokładne: `example.com` pasuje tylko do `example.com`.
- Gwiazdka wieloznaczna: `*` pasuje do każdej domeny.
- Glob subdomen: `*.example.com` pasuje do `blog.example.com`, `forum.example.com`, ale nie do samego `example.com`.

Domena w ładunku to pierwszy niepusty wynik z: pola `domain` komentarza, URL sparsowanego względem konfiguracji domen Twojego tenanta, lub wyszukania `Page` po `urlId`.

### Filtrowanie per-agent

Pole **Agents** pozwala webhookowi subskrybować tylko wybranych agentów. Puste oznacza "wszyscy agenci". Gdy jest wypełnione, webhook wyzwala się tylko dla agentów z listy.

To przydatne, gdy masz jeden webhook dla zdarzeń moderacji, a drugi dla zdarzeń zaangażowania, oba kierowane do różnych systemów docelowych.

### Test send

Interfejs konfiguracji webhooka ma przycisk **Test send**, który wysyła fałszywy ładunek na URL, dzięki czemu możesz zweryfikować łączność, podpisywanie oraz kod odpowiedzi swojego endpointu bez czekania na prawdziwe zdarzenie.

### Logi dostarczania

Każde dostarczenie (i każde ponowienie) trafia na stronę [Webhook Delivery Logs](#webhook-logs), dzięki czemu możesz zobaczyć, co wydarzyło się "na przewodzie".

### Podpisywanie

Każdy webhook jest podpisywany algorytmem HMAC-SHA256 z użyciem sekretu API Twojego tenanta. Zobacz [Webhook Signing](#webhook-signing).

### Kwalifikowalność

Webhooki agentów wymagają ważnego rozliczenia na tenancie. Korzystają z tej samej infrastruktury podpisywania/sekretów co Twoje istniejące webhooki komentarzy - jeśli już zintegrowałeś webhooki komentarzy (zobacz [przewodnik Webhooks](/guide-webhooks.html)), integracja webhooków agentów ma ten sam kształt, tylko z nowymi typami zdarzeń.