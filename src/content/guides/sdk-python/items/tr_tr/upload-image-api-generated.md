Upload and resize an image
===========================

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| sizePreset | string | query | Hayır | Boyut ön ayarı: "Default" (1000x1000px) veya "CrossPlatform" (popüler cihazlar için boyutları oluşturur) |
| urlId | string | query | Hayır | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırmak için |

## Response

Döndürür: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Example

[inline-code-attrs-start title = 'upload_image Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin bir listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API sınıfının bir örneğini oluştur
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Boyut ön ayarı: \"Default\" (1000x1000px) veya \"CrossPlatform\" (popüler cihazlar için boyutları oluşturur) (isteğe bağlı)
    url_id = 'url_id_example' # str | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırmak için (isteğe bağlı)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---