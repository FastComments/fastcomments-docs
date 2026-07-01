Upload and resize an image
============================

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Prednastavitev velikosti: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave) |
| urlId | string | query | No | ID strani, s katere se izvaja nalaganje, za konfiguracijo |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Example

[inline-code-attrs-start title = 'upload_image Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Glej configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopi v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvari instanco API razreda
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Prednastavitev velikosti: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave) (optional)
    url_id = 'url_id_example' # str | ID strani, s katere se izvaja nalaganje, za konfiguracijo (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---