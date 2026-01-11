Завантажити та змінити розмір зображення

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| sizePreset | string | query | Ні | Параметр розміру: "Default" (1000x1000px) або "CrossPlatform" (створює розміри для популярних пристроїв) |
| urlId | string | query | Ні | Ідентифікатор сторінки, з якої відбувається завантаження, для конфігурації |

## Відповідь

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням має значення https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Параметр розміру: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) (необов'язково)
    url_id = 'url_id_example' # str | Ідентифікатор сторінки, з якої відбувається завантаження, для конфігурації (необов'язково)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]