## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| largeInternalURLSanitized | string | query | Ja |  |

## Antwort

Gibt zurück: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/gif_get_large_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_gif_large Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.gif_get_large_response import GifGetLargeResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und ist standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Erstellen einer Instanz der API-Klasse
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