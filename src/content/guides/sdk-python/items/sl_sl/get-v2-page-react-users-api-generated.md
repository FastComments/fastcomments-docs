## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | pot | Da |  |
| urlId | string | poizvedba | Da |  |
| id | string | poizvedba | Da |  |

## Odgovor

Vrne: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_react_users_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_v2_page_react_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_react_users_response import GetV2PageReactUsersResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
        print("The response of PublicApi->get_v2_page_react_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_react_users: %s\n" % e)
[inline-code-end]