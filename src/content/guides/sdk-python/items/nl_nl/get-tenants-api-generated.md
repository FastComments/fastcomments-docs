## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| meta | string | query | Nee |  |
| skip | number | query | Nee |  |

## Antwoord

Retourneert: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenants Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en heeft standaardwaarde https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters
# configureren in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk auth-mechanisme worden hieronder voorbeelden gegeven; gebruik het voorbeeld dat
# past bij uw auth-geval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaarteken weg hieronder om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]

---