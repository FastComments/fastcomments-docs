Każdy webhook agenta jest podpisywany za pomocą HMAC-SHA256 przy użyciu sekretu API Twojej tenanta. Ten sam schemat podpisywania jest używany dla webhooków komentarzy FastComments - jeśli już je zintegrowałeś, webhooki agenta ponownie wykorzystują ten sam nagłówek podpisu i proces weryfikacji.

### Dlaczego podpisywanie

Bez podpisu atakujący, który zna adres URL Twojego webhooka, mógłby wysyłać POST-ami sfałszowane zdarzenia wyglądające tak, jakby pochodziły od FastComments. Podpisywanie oznacza, że Twój endpoint może zweryfikować autentyczność każdej dostawy przed podjęciem działania.

### Jak działają podpisy

Dla każdej dostawy:

1. Platforma wyszukuje sekret API dla tenanta + dopasowanej domeny (zobacz [Przegląd webhooków](#webhooks-overview)).
2. Wysyła aktualny znacznik czasu Unix (w milisekundach) w nagłówku `X-FastComments-Timestamp`.
3. Oblicza `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (w stylu Stripe) i wysyła wynik jako `sha256=<hex>` w nagłówku `X-FastComments-Signature`.
4. Twój endpoint odczytuje nagłówek znacznika czasu, ponownie oblicza HMAC dla otrzymanego `${timestamp}.${body}`, porównuje z wartością `sha256=<hex>` w nagłówku podpisu i odrzuca niezgodności.

Podpisywana treść to **dokładne bajty** wysłane przez platformę, poprzedzone `${timestamp}.` - Twój weryfikator musi użyć surowej treści żądania, a nie ponownie zserializowanego ciągu JSON (w przeciwnym razie kolejność kluczy i białe znaki byłyby różne).

### Sekret API

Ten sam sekret API używany przez [webhooki komentarzy](/guide-webhooks.html). Jest przypisany do pary (tenant, domena) i zarządzany w ustawieniach API Twojego tenanta. Jeśli zmienisz sekret, powinieneś ponownie wdrożyć swój weryfikator, aby odczytał nową wartość przed następną dostawą.

Gdy platforma nie znajdzie **sekretu API** dla dopasowanej domeny, dostawa nie następuje. Dziennik webhooków rejestruje błąd z powodem "no API secret".

### Przykład weryfikacji (Node.js)

[inline-code-attrs-start title = 'Przykład weryfikacji podpisu webhooka'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Używaj `timingSafeEqual` zamiast `===`, aby uniknąć wycieków podpisu przez kanały czasowe.

### Co zawiera podpisana treść

Pełna koperta plus blok `data` specyficzny dla zdarzenia. Zobacz [Ładunki webhooków](#webhook-payloads).

### Zalecenia

- **Weryfikuj przy każdej dostawie.** Jeśli Twój endpoint akceptuje niepodpisane żądania, nie masz żadnej gwarancji integralności.
- **Odrzucaj przy niezgodności podpisu.** Zwracaj 401 lub 403; nie zwracaj 200 OK przy błędnym podpisie, inaczej zamaskujesz ataki w dziennikach dostaw.
- **Używaj HTTPS.** Podpisy chronią integralność; TLS chroni poufność (zarówno Twój sekret, jak i treść komentarza w ładunku).
- **Zmieniaj sekrety** gdy odchodzą członkowie zespołu mający do nich dostęp, lub według harmonogramu.

### Ochrona przed powtórkami (replay)

Samo podpisywanie nie zapobiega atakom typu replay - atakujący, który przechwycił prawdziwą podpisaną dostawę, może wysłać ją ponownie. Ochrona przed powtórkami zależy od Twojego endpointu:

- Użyj pola koperty `occurredAt` i odrzucaj dostawy starsze niż, powiedzmy, 5 minut.
- Użyj `triggerId` lub `approvalId` jako klucza deduplikacji - jeśli już je przetworzyłeś, zignoruj duplikat.

### Zobacz także

- [Przegląd webhooków](#webhooks-overview).
- [Ładunki webhooków](#webhook-payloads).
- Główny [przewodnik webhooków](/guide-webhooks.html) dla szerszej infrastruktury podpisywania.