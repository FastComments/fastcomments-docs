---
Bulk-gebruikersinformatie voor een tenant. Gegeven userIds, retourneert weergave-informatie van User / SSOUser.
Gebruikt door de commentaarwidget om gebruikers die net verschenen zijn via een presence-event te verrijken.
Geen pagina-context: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | pad | Ja |  |
| ids | string | query | Ja | Door komma's gescheiden userIds. |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_users_info Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaardwaarde is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | Door komma's gescheiden userIds.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]

---