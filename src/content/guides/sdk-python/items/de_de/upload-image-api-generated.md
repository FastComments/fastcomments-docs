---
Bild hochladen und skalieren

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Größen-Vorgabe: "Default" (1000x1000px) oder "CrossPlatform" (erstellt Größen für gängige Geräte) |
| urlId | string | query | No | Seiten-ID, von der der Upload stattfindet, zur Konfiguration |

## Antwort

Gibt zurück: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Beispiel

[inline-code-attrs-start title = 'upload_image Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Größen-Vorgabe: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für gängige Geräte) (optional)
    url_id = 'url_id_example' # str | Seiten-ID, von der der Upload stattfindet, zur Konfiguration (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---