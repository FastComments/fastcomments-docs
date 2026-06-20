---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderators_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_moderators Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderators_response import GetModeratorsResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elke authenticatiemethode worden hieronder voorbeelden gegeven; gebruik het voorbeeld dat
# voldoet aan uw authenticatiebehoefte.
# Configureer API-sleutelautorisatie: api_key
# Haal de commentaarkarakter (#) hieronder weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse aan
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_moderators(tenant_id, skip=skip)
        print("The response of DefaultApi->get_moderators:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderators: %s\n" % e)
[inline-code-end]

---