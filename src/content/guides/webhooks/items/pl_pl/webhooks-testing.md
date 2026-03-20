W panelu administracyjnym Webhooks znajdują się przyciski `Send Test Payload` dla każdego typu zdarzenia (Create, Update, Delete). Zdarzenia Create i Update wysyłają przykładowy obiekt WebhookComment, natomiast testowanie Delete wyśle przykładowe ciało żądania zawierające tylko identyfikator.

## Weryfikacja ładunków

Podczas testowania integracji webhook sprawdź, czy przychodzące żądania zawierają następujące nagłówki:

1. **`token`** - Twój sekret API
2. **`X-FastComments-Timestamp`** - znacznik czasu Unix (sekundy)
3. **`X-FastComments-Signature`** - podpis HMAC-SHA256

Użyj weryfikacji podpisu HMAC, aby upewnić się, że ładunki są autentyczne.

## Narzędzia do testowania

Możesz użyć narzędzi takich jak [webhook.site](https://webhook.site) lub [ngrok](https://ngrok.com), aby sprawdzać przychodzące ładunki webhooków podczas tworzenia.

## Typy zdarzeń

- **Create Event**: Wywoływane, gdy zostanie utworzony nowy komentarz. Domyślna metoda: PUT
- **Update Event**: Wywoływane, gdy komentarz zostanie edytowany. Domyślna metoda: PUT
- **Delete Event**: Wywoływane, gdy komentarz zostanie usunięty. Domyślna metoda: DELETE

Każde zdarzenie zawiera pełne dane komentarza w ciele żądania (zobacz [Struktury danych](/guide-webhooks.html#webhooks-structures) dla formatu ładunku).