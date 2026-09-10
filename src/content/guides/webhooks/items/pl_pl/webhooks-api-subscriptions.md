Webhooks można również zarządzać za pośrednictwem REST API. Tak integracje takie jak Zapier subskrybują
zdarzenia komentarzy bez konieczności używania panelu i stosują wzorzec REST Hooks: subskrybuj,
odbieraj zdarzenia, wypisz się.

Subskrypcje API współistnieją z webhookami skonfigurowanymi w panelu. Zdarzenie komentarza jest dostarczane
do każdego webhooka, którego domena pasuje, każdy jako osobna dostawa, niezależnie od tego, w jaki sposób webhook został utworzony.

## Uwierzytelnianie

Każde żądanie wymaga Twojego klucza API w nagłówku `x-api-key` (lub parametru zapytania `API_KEY`) oraz
identyfikatora najemcy w parametrze zapytania `tenantId`. Oba są wyświetlane na stronie API Secret w panelu.

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

| Pole | Wymagane | Opis |
|------|----------|------|
| `url` | Tak | Absolutny adres http lub https. |
| `event` | Tak | `comment-created`, `comment-updated` lub `comment-deleted`. |
| `domain` | Nie | Domena z konfiguracji Twojego konta. Domyślnie `*`, co odbiera zdarzenia ze wszystkich domen. |
| `method` | Nie | `POST` (domyślnie), `PUT` lub `DELETE`. |

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

Subskrybowanie tego samego URL do tego samego zdarzenia i domeny ponownie zwraca istniejącą subskrypcję zamiast
tworzyć duplikat, więc klient może bezpiecznie ponowić próbę. Każdy najemca może mieć maksymalnie 50 subskrypcji API.

## Lista

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Zwraca wszystkie webhooki dla najemcy, w tym te zarządzane w panelu (`"source": "dashboard"`).
Filtrowanie za pomocą `event`, `domain` lub `source`.

## Anulowanie subskrypcji

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Usunięcie subskrypcji usuwa także wszelkie zdarzenia, które nadal są w kolejce. Tylko subskrypcje utworzone
przez API mogą być usunięte w ten sposób. Webhooki z panelu są edytowane na stronie Webhooks.

## Ładunki i podpisy

Dostawy używają tego samego ładunku co webhooki z panelu (zobacz Struktury Danych) i są podpisane tym samym
schematem HMAC (zobacz Bezpieczeństwo i Tokeny API). Subskrypcje API nigdy nie otrzymują przestarzałego nagłówka `token`,
dlatego należy weryfikować nagłówek `X-FastComments-Signature`.

## Przykładowe ładunki

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Zwraca najnowsze komentarze konta w dokładnym kształcie, w jakim dostawa je przenosi, dzięki czemu integracja może
pokazać rzeczywiste przykładowe dane przed przyjściem pierwszego zdarzenia. `event` jest opcjonalny i jedynie walidowany,
ponieważ każde zdarzenie dostarcza ten sam obiekt komentarza. `limit` domyślnie wynosi 3 i przyjmuje wartości od 1 do 10. Kosztuje 2 kredyty API.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Odpowiadanie kodem 410 Gone

Jeśli punkt końcowy subskrypcji API odpowie kodem HTTP `410 Gone`, FastComments traktuje to jako
wypisanie się: subskrypcja jest usuwana wraz z jej oczekującymi zdarzeniami i nie są podejmowane dalsze próby dostawy. Webhooki skonfigurowane w panelu nigdy nie są usuwane automatycznie; dla nich 410 jest zwykłym niepowodzeniem. Każdy inny kod niepowodzenia jest ponawiany i ostatecznie wyłącza webhook, jak opisano w sekcji Jak to działa i Obsługa ponownych prób.

## Panel

Subskrypcje API pojawiają się na liście Webhooks ze źródłem **API**, gdzie administrator może je edytować,
wyłączyć, ponownie włączyć lub usunąć.