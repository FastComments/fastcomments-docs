Upload en verklein een afbeelding

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nee | Groottepreset: "Default" (1000x1000px) of "CrossPlatform" (maakt groottes voor populaire apparaten) |
| urlId | string | query | Nee | Pagina-id vanwaar de upload plaatsvindt, om te configureren |

## Response

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'upload_image Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context binnen met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Groottepreset: "Default" (1000x1000px) of "CrossPlatform" (maakt groottes voor populaire apparaten) (optioneel)
    url_id = 'url_id_example' # str | Pagina-id vanwaar de upload plaatsvindt, om te configureren (optioneel)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]