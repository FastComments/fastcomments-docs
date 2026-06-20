## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Vraća: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_success_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_success_response import APIEmptySuccessResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s pravilima sigurnosti API servera.
# Primjeri za svaku metodu autentifikacije prikazani su dolje, upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurirajte autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Poništite komentiranje dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_user_badge(tenant_id, id)
        print("The response of DefaultApi->delete_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_user_badge: %s\n" % e)
[inline-code-end]