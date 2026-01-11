## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Geeft terug: [`PinComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pin_comment200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'pin_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pin_comment200_response import PinComment200Response
from client.rest import ApiException
from pprint import pprint

# Het opgeven van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->pin_comment: %s\n" % e)
[inline-code-end]