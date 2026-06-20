## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Antwoord

Retourneert: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_comment_text Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]