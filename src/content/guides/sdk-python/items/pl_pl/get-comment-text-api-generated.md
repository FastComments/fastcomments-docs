## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| commentId | string | ścieżka | Tak |  |
| editKey | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text200_response import GetCommentText200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]