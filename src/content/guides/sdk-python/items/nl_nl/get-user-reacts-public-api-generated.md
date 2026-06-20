## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | pad | Ja |  |
| postIds | array | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/user_reacts_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_user_reacts_public Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.user_reacts_response import UserReactsResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse aan
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, post_ids=post_ids, sso=sso)
        print("The response of PublicApi->get_user_reacts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_reacts_public: %s\n" % e)
[inline-code-end]

---