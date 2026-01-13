## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| errorId | string | path | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за delete_email_template_render_error'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су примери за сваки метод аутентификације, користите пример који
# одговара вашем случају коришћења аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    error_id = 'error_id_example' # str | 

    try:
        api_response = api_instance.delete_email_template_render_error(tenant_id, id, error_id)
        print("The response of DefaultApi->delete_email_template_render_error:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_email_template_render_error: %s\n" % e)
[inline-code-end]