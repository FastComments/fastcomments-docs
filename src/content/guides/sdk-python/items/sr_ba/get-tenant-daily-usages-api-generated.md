## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| yearNumber | number | query | Не |  |
| monthNumber | number | query | Не |  |
| dayNumber | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Пример

[inline-code-attrs-start title = 'get_tenant_daily_usages Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Постављање параметра host је опционално и подразумевано је на https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре за аутентификацију и ауторизацију
# у складу са безбедносном политиком API сервера.
# Испод су наведени примјери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе.

# Конфигуришите API key ауторизацију: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Откоментирајте доле да подесите префикс (нпр. Bearer) за API key, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (опционо)
    month_number = 3.4 # float |  (опционо)
    day_number = 3.4 # float |  (опционо)
    skip = 3.4 # float |  (опционо)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]