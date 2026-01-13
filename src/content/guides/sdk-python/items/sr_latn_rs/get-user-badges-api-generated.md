## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| badgeId | string | query | Ne |  |
| type | number | query | Ne |  |
| displayedOnComments | boolean | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badges200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badges200_response import GetUserBadges200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Klijent mora da podesi parametre autentifikacije i autorizacije
# in accordance with the API server security policy.
# u skladu sa politikom bezbednosti API servera.
# Examples for each auth method are provided below, use the example that
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# satisfies your auth use case.
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Configure API key authorization: api_key
# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opciono)
    badge_id = 'badge_id_example' # str |  (opciono)
    type = 3.4 # float |  (opciono)
    displayed_on_comments = True # bool |  (opciono)
    limit = 3.4 # float |  (opciono)
    skip = 3.4 # float |  (opciono)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]