## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Ні |  |
| includeByIP | boolean | query | Ні |  |
| includeByEmailDomain | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_pre_ban_summary.py)

## Приклад

[inline-code-attrs-start title = 'Приклад post_bulk_pre_ban_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_pre_ban_params import BulkPreBanParams
from client.models.bulk_pre_ban_summary import BulkPreBanSummary
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове; за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використайте контекст з екземпляром API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    bulk_pre_ban_params = client.BulkPreBanParams() # BulkPreBanParams | 
    include_by_user_id_and_email = True # bool |  (необов'язково)
    include_by_ip = True # bool |  (необов'язково)
    include_by_email_domain = True # bool |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.post_bulk_pre_ban_summary(bulk_pre_ban_params, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->post_bulk_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_bulk_pre_ban_summary: %s\n" % e)
[inline-code-end]