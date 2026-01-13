## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_hash_tag200_response.py)

## Beispiel

[inline-code-attrs-start title = 'patch_hash_tag Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_hash_tag200_response import PatchHashTag200Response
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.rest import ApiException
from pprint import pprint

# Die Definition des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitspolitik des API-Servers konfigurieren.
# Beispiele für jede Auth-Methode sind unten aufgeführt; verwenden Sie das Beispiel, das
# Ihren Auth-Anwendungsfall erfüllt.

# API-Schlüssel-Authorisierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls benötigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (optional)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (optional)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]