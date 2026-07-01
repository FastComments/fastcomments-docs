## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## Primer

[inline-code-attrs-start title = 'post_set_comment_text Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentTextOptions
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
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
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    broadcast_id = 'broadcast_id_example' # str |  (opciono)
    sso = 'sso_example' # str |  (opciono)

    try:
        api_response = api_instance.post_set_comment_text(tenant_id, comment_id, set_comment_text_params, PostSetCommentTextOptions(broadcast_id=broadcast_id, sso=sso))
        print("Odgovor ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom poziva ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]