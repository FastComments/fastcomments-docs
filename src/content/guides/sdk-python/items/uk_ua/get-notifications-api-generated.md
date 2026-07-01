## Parameters

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |
| skip | number | query | No |  |

## Відповідь

Повертає: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationsOptions
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов’язковим і за замовчуванням встановлює https://fastcomments.com
# Див. configuration.py для отримання списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведено нижче, використайте приклад,
# який відповідає вашому випадку використання.
 
# Налаштування авторизації за допомогою API‑ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API‑ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з інстансом API‑клієнта
with client.ApiClient(configuration) as api_client:
    # Створити інстанс класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов’язковий)
    url_id = 'url_id_example' # str |  (необов’язковий)
    from_comment_id = 'from_comment_id_example' # str |  (необов’язковий)
    viewed = True # bool |  (необов’язковий)
    type = 'type_example' # str |  (необов’язковий)
    skip = 3.4 # float |  (необов’язковий)

    try:
        api_response = api_instance.get_notifications(tenant_id, GetNotificationsOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip))
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]