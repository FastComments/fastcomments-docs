---
## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentIds | string | query | Tak | Lista identyfikatorów komentarzy rozdzielona przecinkami. |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/check_blocked_comments_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.check_blocked_comments_response import CheckBlockedCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Lista identyfikatorów komentarzy rozdzielona przecinkami.
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]

---