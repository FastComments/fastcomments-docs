Carica e ridimensiona un'immagine

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset di dimensione: "Default" (1000x1000px) o "CrossPlatform" (crea dimensioni per i dispositivi più diffusi) |
| urlId | string | query | No | ID della pagina da cui avviene il caricamento, da configurare |

## Risposta

Restituisce: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Esempio

[inline-code-attrs-start title = 'upload_image Esempio'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
# Creare un'istanza della classe API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Preset di dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per i dispositivi più diffusi) (opzionale)
    url_id = 'url_id_example' # str | ID della pagina da cui avviene il caricamento, da configurare (opzionale)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---