## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |

## Respons

Retourneert: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'add_hash_tag Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratie‑parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie‑ en autorisatie‑parameters configureren
# in overeenstemming met het beveiligingsbeleid van de API‑server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven, gebruik het voorbeeld dat
# aan uw authenticatie‑use‑case voldoet.

# Configureer API‑sleutel autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Verwijder de commentaar bij onderstaande regel om een prefix (bijv. Bearer) voor de API‑sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Betreed een context met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]