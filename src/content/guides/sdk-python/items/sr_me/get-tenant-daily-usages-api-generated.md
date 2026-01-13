## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| yearNumber | number | query | Не |  |
| monthNumber | number | query | Не |  |
| dayNumber | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages200_response.py)

## Пример

[inline-code-attrs-start title = 'get_tenant_daily_usages Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages200_response import GetTenantDailyUsages200Response
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опционално и по подразумеваној вредности је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и овлашћења
# у складу са сигурносном политиком API сервера.
# Примери за сваки метод аутентификације су наведени испод, користите пример који
# одговара вашем случају употребе аутентификације.

# Конфигуришите API кључ за ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментујте испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Направите инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (optional)
    month_number = 3.4 # float |  (optional)
    day_number = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]