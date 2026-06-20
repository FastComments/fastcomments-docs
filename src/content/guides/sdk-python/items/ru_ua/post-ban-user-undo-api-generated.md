## Параметры

| Имя | Тип | Location | Обязательный | Описание |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример post_ban_user_undo'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.ban_user_undo_params import BanUserUndoParams
from client.rest import ApiException
from pprint import pprint

# Задание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    ban_user_undo_params = client.BanUserUndoParams() # BanUserUndoParams | 
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.post_ban_user_undo(ban_user_undo_params, sso=sso)
        print("The response of ModerationApi->post_ban_user_undo:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_undo: %s\n" % e)
[inline-code-end]