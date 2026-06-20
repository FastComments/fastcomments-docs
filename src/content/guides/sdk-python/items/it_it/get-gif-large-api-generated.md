## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| largeInternalURLSanitized | string | query | Yes |  |

## Risposta

Restituisce: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/gif_get_large_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_gif_large'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.gif_get_large_response import GifGetLargeResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per default è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    large_internal_url_sanitized = 'large_internal_url_sanitized_example' # str | 

    try:
        api_response = api_instance.get_gif_large(tenant_id, large_internal_url_sanitized)
        print("The response of PublicApi->get_gif_large:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gif_large: %s\n" % e)
[inline-code-end]