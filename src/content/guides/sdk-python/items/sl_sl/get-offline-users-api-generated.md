Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (čisti na strežniku). |
| afterName | string | query | No | Kursor: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kursor za razreševanje neenakosti: posredujte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da pri enakih imenih ne izpustimo vnosov. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL strani (čisti na strežniku).
    after_name = 'after_name_example' # str | Kursor: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
    after_user_id = 'after_user_id_example' # str | Kursor za razreševanje neenakosti: posredujte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da pri enakih imenih ne izpustimo vnosov. (neobvezno)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]