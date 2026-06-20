## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Yes |  |
| tenantId | string | query | No |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Beispiel

[inline-code-attrs-start title = 'delete_hash_tag Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.delete_hash_tag_request_body import DeleteHashTagRequestBody
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Unten auskommentieren, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls benötigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (optional)
    delete_hash_tag_request_body = client.DeleteHashTagRequestBody() # DeleteHashTagRequestBody |  (optional)

    try:
        api_response = api_instance.delete_hash_tag(tag, tenant_id=tenant_id, delete_hash_tag_request_body=delete_hash_tag_request_body)
        print("The response of DefaultApi->delete_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_hash_tag: %s\n" % e)
[inline-code-end]