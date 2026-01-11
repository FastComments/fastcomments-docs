## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_page_api_response.py)

## Primer

[inline-code-attrs-start title = 'add_page Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_page_api_response import AddPageAPIResponse
from client.models.create_api_page_data import CreateAPIPageData
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# U nastavku su dati primeri za svaki metod autentifikacije, iskoristite primer koji
# odgovara vašem slučaju upotrebe.
 
# Konfigurišite autorizaciju preko API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
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