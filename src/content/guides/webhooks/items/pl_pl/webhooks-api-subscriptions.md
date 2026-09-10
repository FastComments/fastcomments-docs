Webhooks można również zarządzać za pośrednictwem REST API. Tak integracje takie jak Zapier subskrybują zdarzenia komentarzy bez użycia panelu i stosują wzorzec REST Hooks: subskrybuj, odbieraj zdarzenia, anuluj subskrypcję.

Subskrypcje API współistnieją z webhookami skonfigurowanymi w panelu. Zdarzenie komentarza jest dostarczane do webhooka panelu dla jego domeny oraz do każdej subskrypcji API, która pasuje, każda jako osobna dostawa. Nie ma limitu jednego subskrybenta na zdarzenie.

## Uwierzytelnianie

Każde żądanie wymaga Twojego klucza API w nagłówku `x-api-key` (lub parametru zapytania `API_KEY`) oraz identyfikatora najemcy w parametrze zapytania `tenantId`. Oba są wyświetlane na stronie API Secret w panelu.

## Subskrypcja

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Pole   | Wymagane | Opis |
|--------|----------|------|
| `url`  | Tak      | Absolutny adres URL http lub https. |
| `event`| Tak      | `comment-created`, `comment-updated` lub `comment-deleted`. |
| `domain`| Nie     | Domena z konfiguracji Twojego konta. Domyślnie `*`, co oznacza odbieranie zdarzeń ze wszystkich domen. |
| `method`| Nie     | `POST` (domyślnie), `PUT` lub `DELETE`. |

Odpowiedź zawiera subskrypcję:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Subskrybowanie tego samego URL do tego samego zdarzenia i domeny ponownie zwraca istniejącą subskrypcję zamiast tworzyć duplikat, więc klient może bezpiecznie ponowić żądanie. Każdy najemca może mieć maksymalnie 50 subskrypcji API.

## Lista

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Zwraca wszystkie webhooki dla najemcy, w tym te zarządzane w panelu (`"source": "dashboard"`). Filtruj przy pomocy `event`, `domain` lub `source`.

## Anulowanie subskrypcji

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Usunięcie subskrypcji powoduje również odrzucenie wszelkich zdarzeń, które nadal czekają w kolejce. Tylko subskrypcje utworzone przez API mogą być usunięte w ten sposób. Webhooki z panelu są edytowane na stronie Webhooks.

## Ładunki i podpisy

Dostawy używają takiego samego ładunku jak webhooki panelu (zobacz Struktury danych) i są podpisane tym samym schematem HMAC (zobacz Bezpieczeństwo i tokeny API). Subskrypcje API nigdy nie otrzymują przestarzałego nagłówka `token`, więc weryfikuj nagłówek `X-FastComments-Signature`.

## Odpowiadanie kodem 410 Gone

Jeśli punkt końcowy subskrypcji API odpowie kodem HTTP `410 Gone`, FastComments traktuje to jako anulowanie subskrypcji: subskrypcja jest usuwana wraz z oczekującymi zdarzeniami i nie są podejmowane dalsze próby dostarczenia. Webhooki skonfigurowane w panelu nigdy nie są usuwane automatycznie; dla nich 410 oznacza zwykłą awarię. Każdy inny kod błędu jest ponawiany i ostatecznie wyłącza webhook, jak opisano w sekcji Jak to działa i Obsługa ponownych prób.

## Panel

Subskrypcje API są wyświetlane na stronie Webhooks pod domeną, dla której zostały utworzone, gdzie administrator może je wyłączyć, ponownie włączyć lub usunąć.