## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| state | number | query | Ne |  |
| skip | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vrne: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## Primer

[inline-code-attrs-start title = 'get_tickets Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre preverjanja pristnosti in avtorizacije
# v skladu s politiko varnosti strežnika API.
# Spodaj so navedeni primeri za vsako metodo avtentikacije, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Nastavite avtorizacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje, če želite nastaviti predpono (npr. Bearer) za ključ API, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    state = 3.4 # float |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)
    limit = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]