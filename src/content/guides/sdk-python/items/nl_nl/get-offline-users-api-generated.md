Eerdere reageerders op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit na het uitputten van /users/online om een "Leden"-sectie weer te geven.
Cursorpaginering op commenterName: de server loopt via $gt door de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit, zonder $skip-kosten.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificatie (serverzijde opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door vanuit de vorige respons. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId door vanuit de vorige respons. Verplicht wanneer afterName is ingesteld zodat gelijke namen geen items verliezen. |

## Respons

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_offline_users Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Pagina-URL-identificatie (serverzijde opgeschoond).
    after_name = 'after_name_example' # str | Cursor: geef nextAfterName door vanuit de vorige respons. (optional)
    after_user_id = 'after_user_id_example' # str | Cursor tiebreaker: geef nextAfterUserId door vanuit de vorige respons. Verplicht wanneer afterName is ingesteld zodat gelijke namen geen items verliezen. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]