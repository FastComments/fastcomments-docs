Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrešavanje neravnoteže: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako bi se imena sa istim vrednostima ne izostavljala. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Primer get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (čišćen na serveru).
    after_name = 'after_name_example' # str | Kursor: prosledite nextAfterName iz prethodnog odgovora. (optional)
    after_user_id = 'after_user_id_example' # str | Kursor za razrešavanje neravnoteže: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako bi se imena sa istim vrednostima ne izostavljala. (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("Odgovor PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak pri pozivu PublicApi->get_online_users: %s\n" % e)
[inline-code-end]