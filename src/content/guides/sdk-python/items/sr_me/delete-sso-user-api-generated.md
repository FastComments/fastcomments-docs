## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| deleteComments | boolean | query | Не |  |
| commentDeleteMode | string | query | Не |  |

## Одговор

Враћа: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Пример

[inline-code-attrs-start title = 'Пример delete_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора да конфигурише параметре аутентификације и ауторизације
# у складу са политиком безбедности API сервера.
# Испод су наведени примери за сваки метод аутентификације, користите пример који
# одговара вашем случају употребе аутентификације.

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
    delete_comments = True # bool |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]