## Параметри

| Назва | Type | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| urlId | string | query | Ні |  |
| fromCommentId | string | query | Ні |  |
| viewed | boolean | query | Ні |  |
| type | string | query | Ні |  |

## Відповідь

Повертає: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведено нижче, використайте приклад, який
# відповідає вашому випадку використання автентифікації.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    from_comment_id = 'from_comment_id_example' # str |  (optional)
    viewed = True # bool |  (optional)
    type = 'type_example' # str |  (optional)

    try:
        api_response = api_instance.get_notification_count(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type)
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]