## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## Primer

[inline-code-attrs-start title = 'Primer block_from_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.block_success import BlockSuccess
from client.models.public_block_from_comment_params import PublicBlockFromCommentParams
from client.rest import ApiException
from pprint import pprint

# Definisanje host-a je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Napravite instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    public_block_from_comment_params = client.PublicBlockFromCommentParams() # PublicBlockFromCommentParams | 
    sso = 'sso_example' # str |  (opciono)

    try:
        api_response = api_instance.block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso=sso)
        print("The response of PublicApi->block_from_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->block_from_comment_public: %s\n" % e)
[inline-code-end]