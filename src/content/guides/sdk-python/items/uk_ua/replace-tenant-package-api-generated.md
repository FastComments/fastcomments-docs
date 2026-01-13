## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Приклад

[inline-code-attrs-start title = 'replace_tenant_package Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.replace_tenant_package_body import ReplaceTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов'язковим і за замовчуванням встановлено на https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Наведені нижче приклади для кожного методу автентифікації — використайте той
# що відповідає вашому сценарію автентифікації.

# Налаштуйте авторизацію API-ключем: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть у контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    replace_tenant_package_body = client.ReplaceTenantPackageBody() # ReplaceTenantPackageBody | 

    try:
        api_response = api_instance.replace_tenant_package(tenant_id, id, replace_tenant_package_body)
        print("The response of DefaultApi->replace_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->replace_tenant_package: %s\n" % e)
[inline-code-end]

---