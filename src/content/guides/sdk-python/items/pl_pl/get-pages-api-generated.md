## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pages_api_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_pages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pages_api_response import GetPagesAPIResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
# The client must configure the authentication and authorization parameters
# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# in accordance with the API server security policy.
# zgodnie z polityką bezpieczeństwa serwera API.
# Examples for each auth method are provided below, use the example that
# Przykłady dla każdej metody uwierzytelniania znajdują się poniżej, użyj przykładu który
# satisfies your auth use case.
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.
# Configure API key authorization: api_key
# Konfiguracja autoryzacji klucza API: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, w razie potrzeby

# Enter a context with an instance of the API client
# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_pages(tenant_id)
        print("The response of DefaultApi->get_pages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pages: %s\n" % e)
[inline-code-end]