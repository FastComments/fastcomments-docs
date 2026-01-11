Отпреми и промени величину слике

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Пресет величине: "Default" (1000x1000px) или "CrossPlatform" (креира величине за популарне уређаје) |
| urlId | string | query | No | Ид странице са које се врши отпремање, за конфигурацију |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Пример

[inline-code-attrs-start title = 'Пример upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Пресет величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (креира величине за популарне уређаје) (опционо)
    url_id = 'url_id_example' # str | Ид странице са које се врши отпремање, за конфигурацију (опционо)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]