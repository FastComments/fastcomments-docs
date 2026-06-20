---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Da |  |
| tenantId | string | query | Ne |  |

## Odgovor

Vraća: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Primjer

[inline-code-attrs-start title = 'patch_hash_tag Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora podesiti parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaku metodu autentifikacije su navedeni ispod, koristite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite donji red da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (optional)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (optional)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]

---