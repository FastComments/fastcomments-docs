## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postId | string | path | Da |  |
| isUndo | boolean | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_public200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer react_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_public200_response import ReactFeedPostPublic200Response
from client.rest import ApiException
from pprint import pprint

# Nastavitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (neobvezno)
    broadcast_id = 'broadcast_id_example' # str |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, is_undo=is_undo, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]