## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| tag | string | path | Da |  |

## Odgovor

Returns: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Primjer

[inline-code-attrs-start title = 'patch_hash_tag Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumijeva https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurisati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaki metod autentikacije su navedeni ispod, koristite primjer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurišite autorizaciju API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (optional)

    try:
        api_response = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]