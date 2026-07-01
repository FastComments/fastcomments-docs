## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| urlId | string | query | Ні |  |
| fromCommentId | string | query | Ні |  |
| viewed | boolean | query | Ні |  |
| type | string | query | Ні |  |

## Відповідь

Повертає: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Приклад

[inline-code-attrs-start title = 'get_notification_count Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationCountOptions
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов’язковим і за замовчуванням встановлює https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Приклади для кожного методу автентифікації наведені нижче, використайте той приклад,
# який відповідає вашому випадку використання автентифікації.

# Налаштування авторизації за допомогою API‑ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб задати префікс (наприклад, Bearer) для API‑ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Відкрийте контекст з екземпляром API‑клієнта
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
        api_response = api_instance.get_notification_count(tenant_id, GetNotificationCountOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type))
        print("Відповідь DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Виняток при виклику DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]