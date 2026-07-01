## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| deleteComments | boolean | query | Nee |  |
| commentDeleteMode | string | query | Nee |  |

## Response

Retourneert: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'delete_sso_user Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en default naar https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratie‑parameters.
# De client moet de authenticatie‑ en autorisatie‑parameters configureren
# in overeenstemming met het beveiligingsbeleid van de API‑server.
# Voorbeelden voor elke auth‑methode worden hieronder gegeven, gebruik het voorbeeld dat
# voldoet aan jouw authenticatie‑geval.

# Configureer API‑sleutel autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment hieronder om een prefix (bijv. Bearer) voor de API‑sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Voer een context in met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]