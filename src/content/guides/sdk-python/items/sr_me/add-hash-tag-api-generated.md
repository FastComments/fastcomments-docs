## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |

## Response

Returns: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Example

[inline-code-attrs-start title = 'add_hash_tag Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumeva https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfigurise parametre za autentifikaciju i autorizaciju
# u skladu sa sigurnosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# zadovoljava vašu potrebu za autentifikacijom.

# Konfigurišite autorizaciju API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
        print("Odgovor DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak pri pozivanju DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]