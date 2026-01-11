## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| commentId | string | ścieżka | Tak |  |
| broadcastId | string | zapytanie | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`PinComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pin_comment200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład pin_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pin_comment200_response import PinComment200Response
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->pin_comment: %s\n" % e)
[inline-code-end]

---