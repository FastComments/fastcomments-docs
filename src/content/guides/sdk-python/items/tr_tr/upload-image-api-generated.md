Bir resmi yükleyin ve yeniden boyutlandırın

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Boyut ön ayarı: "Default" (1000x1000px) veya "CrossPlatform" (popüler cihazlar için boyutlar oluşturur) |
| urlId | string | query | No | Yüklemenin yapıldığı sayfanın kimliği, yapılandırmak için |

## Dönen değer

Dönen değer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Örnek

[inline-code-attrs-start title = 'upload_image Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneği ile bir bağlam içinde çalışın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Boyut ön ayarı: \"Default\" (1000x1000px) veya \"CrossPlatform\" (popüler cihazlar için boyutlar oluşturur) (isteğe bağlı)
    url_id = 'url_id_example' # str | Yüklemenin yapıldığı sayfanın kimliği, yapılandırmak için (isteğe bağlı)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]