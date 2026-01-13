---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| urlId | string | query | Nee |  |
| fromCommentId | string | query | Nee |  |
| viewed | boolean | query | Nee |  |
| type | string | query | Nee |  |
| skip | number | query | Nee |  |

## Antwoord

Retourneert: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_notifications Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications200_response import GetNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven, gebruik het voorbeeld dat
# past bij uw authenticatie-geval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaartekens weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optioneel)
    url_id = 'url_id_example' # str |  (optioneel)
    from_comment_id = 'from_comment_id_example' # str |  (optioneel)
    viewed = True # bool |  (optioneel)
    type = 'type_example' # str |  (optioneel)
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]

---