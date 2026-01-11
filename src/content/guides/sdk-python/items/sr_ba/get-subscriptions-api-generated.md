## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_subscriptions_api_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_subscriptions'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_subscriptions_api_response import GetSubscriptionsAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# in accordance with the API server security policy.
# u skladu sa politikom sigurnosti API servera.
# Examples for each auth method are provided below, use the example that
# Primjeri za svaki metod autentifikacije su dati ispod, upotrijebite primjer koji
# satisfies your auth use case.
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Configure API key authorization: api_key
# Konfigurišite autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)

    try:
        api_response = api_instance.get_subscriptions(tenant_id, user_id=user_id)
        print("The response of DefaultApi->get_subscriptions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_subscriptions: %s\n" % e)
[inline-code-end]