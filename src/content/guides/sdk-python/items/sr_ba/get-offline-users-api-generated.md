Prošli komentatori na stranici koji trenutno nisu online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije 'Članovi'.
Kursor paginacija na commenterName: server prolazi parcijalni indeks {tenantId, urlId, commenterName} od afterName naprijed preko $gt, bez troška $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tajbrejker za kursor: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen tako da se zbog jednakih imena unosi ne izgube. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'Primjer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
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
    url_id = 'url_id_example' # str | Identifikator URL stranice (očišćen na serverskoj strani).
    after_name = 'after_name_example' # str | Kursor: pošaljite nextAfterName iz prethodnog odgovora. (opcionalno)
    after_user_id = 'after_user_id_example' # str | Tajbrejker za kursor: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen tako da se zbog jednakih imena unosi ne izgube. (opcionalno)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]