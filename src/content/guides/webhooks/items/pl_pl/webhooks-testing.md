W panelu administracyjnym Webhooks znajdują się przyciski `Send Test Payload` dla każdego typu zdarzenia (Utworzenie, Aktualizacja, Usunięcie). Zdarzenia Utworzenia i Aktualizacji wysyłają przykładowy obiekt WebhookComment, natomiast testowanie Usunięcia wyśle przykładową treść żądania zawierającą tylko identyfikator.

## Weryfikacja treści żądań

Podczas testowania integracji webhooków upewnij się, że przychodzące żądania zawierają następujące nagłówki:

1. **`token`** - Twój sekret API
2. **`X-FastComments-Timestamp`** - Znacznik czasu Unix (w sekundach)
3. **`X-FastComments-Signature`** - Sygnatura HMAC-SHA256

Użyj weryfikacji sygnatury HMAC, aby upewnić się, że treści żądań są autentyczne.

## Narzędzia testowe

Możesz użyć narzędzi takich jak [webhook.site](https://webhook.site) lub [ngrok](https://ngrok.com), aby przejrzeć przychodzące dane webhooków podczas tworzenia.

## Typy zdarzeń

- **Zdarzenie utworzenia**: Wywoływane, gdy nowy komentarz zostanie utworzony. Domyślna metoda: PUT
- **Zdarzenie aktualizacji**: Wywoływane, gdy komentarz jest edytowany. Domyślna metoda: PUT
- **Zdarzenie usunięcia**: Wywoływane, gdy komentarz zostanie usunięty. Domyślna metoda: DELETE

Każde zdarzenie zawiera pełne dane komentarza w treści żądania (zobacz [Struktury danych](/guides/webhooks/webhooks-structures) dla formatu danych).