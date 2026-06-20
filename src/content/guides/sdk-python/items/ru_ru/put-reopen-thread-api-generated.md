## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| urlId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример put_reopen_thread'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Войдите в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.put_reopen_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_reopen_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_reopen_thread: %s\n" % e)
[inline-code-end]