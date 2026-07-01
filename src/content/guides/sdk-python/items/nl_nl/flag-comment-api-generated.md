## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |

## Respons

Retourneert: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'flag_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import FlagCommentOptions
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratie‑parameters.
# De client moet de authenticatie‑ en autorisatie‑parameters configureren
# in overeenstemming met het beveiligingsbeleid van de API‑server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven, gebruik het voorbeeld dat
# aan uw authenticatie‑gebruikssituatie voldoet.

# Configureer API‑sleutel autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Verwijder de commentaartekens hieronder om een prefix (bijv. Bearer) voor de API‑sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optioneel)
    anon_user_id = 'anon_user_id_example' # str |  (optioneel)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, FlagCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("De respons van DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception bij het aanroepen van DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]