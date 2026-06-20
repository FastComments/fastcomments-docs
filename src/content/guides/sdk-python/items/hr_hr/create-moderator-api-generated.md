## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_moderator_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer create_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_moderator_body import CreateModeratorBody
from client.models.create_moderator_response import CreateModeratorResponse
from client.rest import ApiException
from pprint import pprint

# Navođenje hosta je neobavezno i po zadanom je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API servera.
# Donje su navedeni primjeri za svaki način autentikacije; koristite onaj
# koji odgovara vašem slučaju upotrebe.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Poništite komentar ispod za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_moderator_body = client.CreateModeratorBody() # CreateModeratorBody | 

    try:
        api_response = api_instance.create_moderator(tenant_id, create_moderator_body)
        print("The response of DefaultApi->create_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_moderator: %s\n" % e)
[inline-code-end]