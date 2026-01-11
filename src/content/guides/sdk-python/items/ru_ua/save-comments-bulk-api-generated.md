## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| isLive | boolean | query | Нет |  |
| doSpamCheck | boolean | query | Нет |  |
| sendEmails | boolean | query | Нет |  |
| populateNotifications | boolean | query | Нет |  |

## Ответ

Возвращает: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comment200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример save_comments_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comment200_response import SaveComment200Response
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно, по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клиент должен настроить параметры аутентификации и авторизации
# в соответствии с политикой безопасности сервера API.
# Примеры для каждого метода аутентификации приведены ниже, используйте
# тот пример, который соответствует вашему сценарию аутентификации.

# Настройка авторизации по API-ключу: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Раскомментируйте ниже для установки префикса (например, Bearer) для API-ключа, если необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Вход в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (необязательно)
    do_spam_check = True # bool |  (необязательно)
    send_emails = True # bool |  (необязательно)
    populate_notifications = True # bool |  (необязательно)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]