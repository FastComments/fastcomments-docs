## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_page_api_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer add_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_page_api_response import AddPageAPIResponse
from client.models.create_api_page_data import CreateAPIPageData
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s pravilnikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije su prikazani u nastavku, upotrijebite primjer koji
# zadovoljava vaš slučaj uporabe autentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_page_data = client.CreateAPIPageData() # CreateAPIPageData | 

    try:
        api_response = api_instance.add_page(tenant_id, create_api_page_data)
        print("The response of DefaultApi->add_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_page: %s\n" % e)
[inline-code-end]