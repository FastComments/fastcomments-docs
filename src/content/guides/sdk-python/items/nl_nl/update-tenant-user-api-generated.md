## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nee |  |

## Antwoord

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'update_tenant_user Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_tenant_user_body import UpdateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode staan hieronder; gebruik het voorbeeld dat
# voldoet aan uw authenticatiegeval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal het commentaar hieronder weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_user_body = client.UpdateTenantUserBody() # UpdateTenantUserBody | 
    update_comments = 'update_comments_example' # str |  (optioneel)

    try:
        api_response = api_instance.update_tenant_user(tenant_id, id, update_tenant_user_body, update_comments=update_comments)
        print("The response of DefaultApi->update_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_user: %s\n" % e)
[inline-code-end]