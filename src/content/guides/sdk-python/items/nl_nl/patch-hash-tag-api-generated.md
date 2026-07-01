## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| tag | string | path | Ja |  |

## Respons

Retourneert: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'patch_hash_tag Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratie‑parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API‑server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven, gebruik het voorbeeld dat
# voldoet aan uw authenticatiegeval.

# Configureer API‑sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Verwijder de commentaar op de onderstaande regel om een prefix (bijv. Bearer) voor de API‑sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (optional)

    try:
        api_response = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]