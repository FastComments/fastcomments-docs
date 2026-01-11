## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | pot | Da |  |
| commentId | string | pot | Da |  |
| broadcastId | string | poizvedba | Da |  |
| editKey | string | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_public200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer delete_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_public200_response import DeleteCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_public: %s\n" % e)
[inline-code-end]