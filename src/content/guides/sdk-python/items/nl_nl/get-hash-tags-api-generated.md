## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nee |  |

## Response

Geeft terug: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_hash_tags Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags200_response import GetHashTags200Response
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters
# configureren in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk authenticatiemethode worden hieronder voorbeelden gegeven; gebruik het
# voorbeeld dat voldoet aan uw authenticatiebehoefte.

# Configureer API key-autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de volgende regel uit commentaar om een prefix (bijv. Bearer) voor de API key in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]