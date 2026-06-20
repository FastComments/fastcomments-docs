## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Odgovor

Vraća: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderators_response.py)

## Primjer

[inline-code-attrs-start title = 'get_moderators Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderators_response import GetModeratorsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opciona i zadana vrijednost je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaki način autentifikacije prikazani su u nastavku; upotrijebite primjer koji
# odgovara vašem načinu autentifikacije.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_moderators(tenant_id, skip=skip)
        print("The response of DefaultApi->get_moderators:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderators: %s\n" % e)
[inline-code-end]