## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates_response.py)

## Приклад

[inline-code-attrs-start title = 'get_email_templates Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates_response import GetEmailTemplatesResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове та за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Нижче наведено приклади для кожного методу автентифікації — використайте той
# який відповідає вашому сценарію використання автентифікації.

# Налаштуйте авторизацію за допомогою API-ключа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (необов'язково)

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]

---