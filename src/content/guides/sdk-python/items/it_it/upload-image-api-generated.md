Carica e ridimensiona un'immagine

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| sizePreset | string | query | No | Preset dimensione: "Default" (1000x1000px) o "CrossPlatform" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID della pagina da cui avviene il caricamento, per configurare |

## Response

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Preset dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari) (opzionale)
    url_id = 'url_id_example' # str | ID della pagina da cui avviene il caricamento, per configurare (opzionale)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]