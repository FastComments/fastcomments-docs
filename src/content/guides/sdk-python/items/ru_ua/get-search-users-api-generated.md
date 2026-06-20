## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_user_search_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_user_search_response import ModerationUserSearchResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно — по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_search_users(value=value, sso=sso)
        print("The response of ModerationApi->get_search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_users: %s\n" % e)
[inline-code-end]