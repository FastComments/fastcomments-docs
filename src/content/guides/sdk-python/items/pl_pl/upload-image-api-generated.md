Upload and resize an image

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset rozmiaru: "Default" (1000x1000px) lub "CrossPlatform" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | No | Id strony, z której odbywa się upload, aby skonfigurować |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Example

[inline-code-attrs-start title = 'Przykład upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie wskazuje https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Preset rozmiaru: "Default" (1000x1000px) lub "CrossPlatform" (tworzy rozmiary dla popularnych urządzeń) (opcjonalnie)
    url_id = 'url_id_example' # str | Id strony, z której odbywa się upload, aby skonfigurować (opcjonalnie)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]