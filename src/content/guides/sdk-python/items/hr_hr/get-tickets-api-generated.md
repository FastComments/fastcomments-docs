## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| state | number | query | Ne |  |
| skip | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vraća: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## Primjer

[inline-code-attrs-start title = 'get_tickets Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s pravilima sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentikacije dani su u nastavku, upotrijebite primjer koji
# zadovoljava vaš scenarij autentikacije.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobavezno)
    state = 3.4 # float |  (neobavezno)
    skip = 3.4 # float |  (neobavezno)
    limit = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]