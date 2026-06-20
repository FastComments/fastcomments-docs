---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |

## Response

Retourneert: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v1_page_likes.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_v1_page_likes Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v1_page_likes import GetV1PageLikes
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v1_page_likes(tenant_id, url_id)
        print("The response of PublicApi->get_v1_page_likes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v1_page_likes: %s\n" % e)
[inline-code-end]

---