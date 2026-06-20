## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| yearNumber | number | query | Ні |  |
| monthNumber | number | query | Ні |  |
| dayNumber | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_tenant_daily_usages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту необов'язкове та за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клієнт повинен налаштувати параметри автентифікації та авторизації
# відповідно до політики безпеки сервера API.
# Наведені нижче приклади для кожного методу автентифікації —
# використайте той, що підходить для вашого сценарію.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Увійдіть в контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр API-класу
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float | (необов'язково)
    month_number = 3.4 # float | (необов'язково)
    day_number = 3.4 # float | (необов'язково)
    skip = 3.4 # float | (необов'язково)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]