## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vrne: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/unblock_success.py)

## Primer

[inline-code-attrs-start title = 'Primer un_block_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.un_block_from_comment_params import UnBlockFromCommentParams
from client.models.unblock_success import UnblockSuccess
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre avtentikacije in avtorizacije
# v skladu z varnostno politiko API strežnika.
# Spodaj so prikazani primeri za vsako metodo avtentikacije; uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    un_block_from_comment_params = client.UnBlockFromCommentParams() # UnBlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (neobvezno)
    anon_user_id = 'anon_user_id_example' # str |  (neobvezno)

    try:
        api_response = api_instance.un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_block_user_from_comment: %s\n" % e)
[inline-code-end]