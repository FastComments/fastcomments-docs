## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |

## Одговор

Враћа: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_page_by_urlid_api_response.py)

## Пример

[inline-code-attrs-start title = 'get_page_by_urlid Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_page_by_urlid_api_response import GetPageByURLIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и ауторизације
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације су дати доле, користите пример који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите ауторизацију API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Уклоните коментар испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_page_by_urlid(tenant_id, url_id)
        print("The response of DefaultApi->get_page_by_urlid:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_page_by_urlid: %s\n" % e)
[inline-code-end]