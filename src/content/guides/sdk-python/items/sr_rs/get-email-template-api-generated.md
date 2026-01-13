---
## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template200_response.py)

## Пример

[inline-code-attrs-start title = 'get_email_template Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template200_response import GetEmailTemplate200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да подеси параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Примери за сваки метод аутентификације су дати испод, користите онај
# који задовољава ваш случај употребе.

# Конфигуришите ауторизацију помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Одукоментарите испод да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_email_template(tenant_id, id)
        print("The response of DefaultApi->get_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template: %s\n" % e)
[inline-code-end]

---