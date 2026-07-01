Upload and resize an image
==========================

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Не | Претподешавање величине: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | Не | Идентификатор странице са које се врши отпремање, за подешавање |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Претподешавање величине: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) (optional)
    url_id = 'url_id_example' # str | Идентификатор странице са које се врши отпремање, за подешавање (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("Одговор PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Изузетак приликом позивања PublicApi->upload_image: %s\n" % e)
[inline-code-end]