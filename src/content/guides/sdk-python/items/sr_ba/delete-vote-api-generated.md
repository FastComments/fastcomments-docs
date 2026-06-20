## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| editKey | string | query | Ne |  |

## Odgovor

Vraća: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint
import os

# Definisanje hosta je opciono i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaku metodu autentifikacije dati su ispod, koristite primjer koji
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
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