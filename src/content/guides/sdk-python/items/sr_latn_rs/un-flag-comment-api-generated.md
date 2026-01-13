## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer un_flag_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment200_response import FlagComment200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Klijent mora podesiti parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Configure API key authorization: api_key
# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (opciono)
    anon_user_id = 'anon_user_id_example' # str |  (opciono)

    try:
        api_response = api_instance.un_flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_flag_comment: %s\n" % e)
[inline-code-end]