Upload og ændr størrelse på et billede

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nej | Størrelsesforudindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser til populære enheder) |
| urlId | string | query | Nej | Side-id, som upload sker fra, til konfiguration |

## Svar

Returnerer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Eksempel

[inline-code-attrs-start title = 'upload_image Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfrit og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Størrelsesforudindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser til populære enheder) (valgfri)
    url_id = 'url_id_example' # str | Side-id, som upload sker fra, til konfiguration (valgfri)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]