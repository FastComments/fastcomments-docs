## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| editKey | string | query | Ne |  |

## Odgovor

Vrača: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Primer

[inline-code-attrs-start title = 'Primer delete_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za overjanje in avtorizacijo
# v skladu s politiko varnosti API strežnika.
# Spodaj so prikazani primeri za vsako metodo overjanja, uporabite primer, ki
# ustreza vašemu primeru uporabe.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)

    try:
        api_response = api_instance.delete_vote(tenant_id, id, edit_key=edit_key)
        print("The response of DefaultApi->delete_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_vote: %s\n" % e)
[inline-code-end]