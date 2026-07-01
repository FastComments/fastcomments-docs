Past коментатори на страници који ТРЕНУТНО НИСУ онлајн. Сортирани по displayName.  
Користите ово након што исцрпите /users/online да прикажете секцију „Members“.  
Курсорска пагинација на commenterName: сервер пролази кроз парцијални {tenantId, urlId, commenterName} индекс од afterName напред помоћу $gt, без $skip трошка.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL‑а странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за решавање везног догађаја: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају истица имена улази не изгубили. |

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

# Definisanje host-a je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara podešavanja.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL‑а странице (очишћен на серверу).
    after_name = 'after_name_example' # str | Курсор: проследите nextAfterName из претходног одговора. (опционално)
    after_user_id = 'after_user_id_example' # str | Курсор за решавање везног догађаја: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају истица имена улази не изгубили. (опционално)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]