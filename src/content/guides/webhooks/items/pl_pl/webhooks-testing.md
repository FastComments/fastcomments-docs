Nowe i edytowane strony webhooków mają przycisk `Send Test Payload`, który wysyła żądanie do adresu URL aktualnie znajdującego się w formularzu, niezależnie od tego, czy został on zapisany. Zdarzenia Create i Update wysyłają przykładowy obiekt **WebhookComment**, natomiast testowanie Delete wyśle przykładowe ciało żądania zawierające jedynie identyfikator.

## Weryfikacja ładunków

Podczas testowania integracji webhooka, sprawdź, czy przychodzące żądania zawierają następujące nagłówki:

1. **`X-FastComments-Timestamp`** – znacznik czasu Unix (sekundy)  
2. **`X-FastComments-Signature`** – podpis HMAC‑SHA256  

Webhooki utworzone przed wprowadzeniem schematu podpisu otrzymują również nagłówek **`token`** zawierający Twój sekret API. Nowe webhooki go nie mają.

Użyj weryfikacji podpisu HMAC, aby zapewnić autentyczność ładunków.

## Narzędzia testowe

Możesz używać narzędzi takich jak [webhook.site](https://webhook.site) lub [ngrok](https://ngrok.com), aby przeglądać przychodzące ładunki webhooków podczas programowania.

## Typy zdarzeń

- **Zdarzenie Create**: wywoływane, gdy zostaje utworzony nowy komentarz.  
- **Zdarzenie Update**: wywoływane, gdy komentarz zostaje edytowany.  
- **Zdarzenie Delete**: wywoływane, gdy komentarz zostaje usunięty.  

Każdy webhook jest powiązany z jednym zdarzeniem i jedną metodą HTTP (POST, PUT lub DELETE). Każde zdarzenie zawiera pełne dane komentarza w ciele żądania (zobacz [Data Structures](/guide-webhooks.html#webhooks-structures) po format ładunku).