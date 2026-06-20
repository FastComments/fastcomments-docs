---
Агрегує документи, групуючи їх (якщо надано groupBy) та застосовуючи кілька операцій.
Підтримуються різні операції (наприклад sum, countDistinct, avg тощо).

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| parentTenantId | string | query | Ні |  |
| includeStats | boolean | query | Ні |  |

## Відповідь

Повертає: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Нижче наведено приклади для кожного методу аутентифікації, використайте приклад, який
# відповідає вашому випадку використання.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно

# Enter a context with an instance of the API client
# Використайте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Створіть екземпляр класу API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (необов'язково)
    include_stats = True # bool |  (необов'язково)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]

---