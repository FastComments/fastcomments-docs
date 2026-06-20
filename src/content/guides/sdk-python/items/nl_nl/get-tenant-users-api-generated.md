## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_users_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant_users Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_users_response import GetTenantUsersResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst met alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elke authenticatiemethode worden hieronder voorbeelden gegeven, gebruik het voorbeeld dat
# past bij uw authenticatiescenario.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder het commentaar weg om een voorvoegsel (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_tenant_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_users: %s\n" % e)
[inline-code-end]