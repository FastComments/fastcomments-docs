## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'update_moderator Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_moderator_body import UpdateModeratorBody
from client.rest import ApiException
from pprint import pprint

# Подешавање параметра host је опционално и подразумевано је https://fastcomments.com
# Видите configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су наведени примјери за сваки метод аутентификације, користите примјер који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите ауторизацију API кључем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_moderator_body = client.UpdateModeratorBody() # UpdateModeratorBody | 

    try:
        api_response = api_instance.update_moderator(tenant_id, id, update_moderator_body)
        print("The response of DefaultApi->update_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_moderator: %s\n" % e)
[inline-code-end]