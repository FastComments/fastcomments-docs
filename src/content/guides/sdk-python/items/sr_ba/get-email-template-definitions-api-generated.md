## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions200_response.py)

## Пример

[inline-code-attrs-start title = 'get_email_template_definitions Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions200_response import GetEmailTemplateDefinitions200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумјевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са сигурносном политиком API сервера.
# Доњи примјери за сваки метод аутентификације су наведени испод, користите примјер који
# одговара вашем случају коришћења аутентификације.

# Конфигуришите овлашћење помоћу API кључа: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, откоментирајте доле да бисте поставили префикс (нпр. Bearer) за API кључ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]