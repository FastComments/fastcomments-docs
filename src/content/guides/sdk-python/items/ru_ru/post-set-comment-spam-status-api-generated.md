## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| spam | boolean | query | Нет |  |
| permNotSpam | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример post_set_comment_spam_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Открываем контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр API-класса
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (необязательно)
    perm_not_spam = True # bool |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.post_set_comment_spam_status(comment_id, spam=spam, perm_not_spam=perm_not_spam, sso=sso)
        print("The response of ModerationApi->post_set_comment_spam_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_spam_status: %s\n" % e)
[inline-code-end]

---