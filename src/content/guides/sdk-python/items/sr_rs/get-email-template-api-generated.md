## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_response.py)

## Пример

[inline-code-attrs-start title = 'get_email_template Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_response import GetEmailTemplateResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# у складу са политиком безбедности API сервера.
# Испод су примери за сваки метод аутентификације; користите онај
# који одговара вашем случају коришћења.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Откоментаришите доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
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