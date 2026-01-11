## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_page_api_response.py)

## Primjer

[inline-code-attrs-start title = 'patch_page Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_page_api_response import PatchPageAPIResponse
from client.models.update_api_page_data import UpdateAPIPageData
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaku metodu autentikacije navedeni su dolje, upotrijebite primjer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_page_data = client.UpdateAPIPageData() # UpdateAPIPageData | 

    try:
        api_response = api_instance.patch_page(tenant_id, id, update_api_page_data)
        print("The response of DefaultApi->patch_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_page: %s\n" % e)
[inline-code-end]

---