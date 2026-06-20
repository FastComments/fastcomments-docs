---
## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_hash_tags'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags_response import GetHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API poslužitelja.
# Primjeri za svaku metodu autentikacije navedeni su u nastavku; upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ako je potrebno, otkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]

---