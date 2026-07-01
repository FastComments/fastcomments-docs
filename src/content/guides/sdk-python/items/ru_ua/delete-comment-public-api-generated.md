## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| broadcastId | string | query | Так |  |
| editKey | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_delete_comment_response.py)

## Приклад

[inline-code-attrs-start title = 'delete_comment_public Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import DeleteCommentPublicOptions
from client.models.public_api_delete_comment_response import PublicAPIDeleteCommentResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов’язкове і за замовчуванням встановлює https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, DeleteCommentPublicOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->delete_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_public: %s\n" % e)
[inline-code-end]