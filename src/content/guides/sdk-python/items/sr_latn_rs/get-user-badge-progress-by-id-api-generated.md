## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badge_progress_by_id200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_user_badge_progress_by_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badge_progress_by_id200_response import GetUserBadgeProgressById200Response
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_id(tenant_id, id)
        print("The response of DefaultApi->get_user_badge_progress_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_id: %s\n" % e)
[inline-code-end]

---