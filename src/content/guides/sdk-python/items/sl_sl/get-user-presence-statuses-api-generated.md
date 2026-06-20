## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlIdWS | string | query | Da |  |
| userIds | string | query | Da |  |

## Odgovor

Vrača: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_presence_statuses_response.py)

## Primer

[inline-code-attrs-start title = 'get_user_presence_statuses Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_presence_statuses_response import GetUserPresenceStatusesResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id_ws = 'url_id_ws_example' # str | 
    user_ids = 'user_ids_example' # str | 

    try:
        api_response = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
        print("The response of PublicApi->get_user_presence_statuses:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_presence_statuses: %s\n" % e)
[inline-code-end]