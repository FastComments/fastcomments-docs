## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlIdWS | string | query | Да |  |
| userIds | string | query | Да |  |

## Ответ

Возвращает: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_presence_statuses200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_presence_statuses'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_presence_statuses200_response import GetUserPresenceStatuses200Response
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id_ws = 'url_id_ws_example' # str | 
    user_ids = 'user_ids_example' # str | 

    try:
        api_response = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
        print("The response of PublicApi->get_user_presence_statuses:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_presence_statuses: %s\n" % e)
[inline-code-end]