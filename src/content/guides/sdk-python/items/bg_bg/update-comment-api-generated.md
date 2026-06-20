## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| contextUserId | string | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| isLive | boolean | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за update_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# Клиентът трябва да конфигурира параметрите за удостоверяване и авторизация
# in accordance with the API server security policy.
# в съответствие с политиката за сигурност на API сървъра.
# Examples for each auth method are provided below, use the example that
# Примерите за всеки метод на удостоверяване са дадени по-долу, използвайте примера който
# satisfies your auth use case.
# удовлетворява вашия случай на използване за удостоверяване.

# Configure API key authorization: api_key
# Конфигурирайте удостоверяването с API ключ: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Премахнете коментара отдолу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Създайте инстанция на API класа
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (optional)
    do_spam_check = True # bool |  (optional)
    is_live = True # bool |  (optional)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]