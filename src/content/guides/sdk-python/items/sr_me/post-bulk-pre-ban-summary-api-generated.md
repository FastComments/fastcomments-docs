## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Не |  |
| includeByIP | boolean | query | Не |  |
| includeByEmailDomain | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_pre_ban_summary.py)

## Пример

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_pre_ban_params import BulkPreBanParams
from client.models.bulk_pre_ban_summary import BulkPreBanSummary
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    bulk_pre_ban_params = client.BulkPreBanParams() # BulkPreBanParams | 
    include_by_user_id_and_email = True # bool |  (опционално)
    include_by_ip = True # bool |  (опционално)
    include_by_email_domain = True # bool |  (опционално)
    sso = 'sso_example' # str |  (опционално)

    try:
        api_response = api_instance.post_bulk_pre_ban_summary(bulk_pre_ban_params, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->post_bulk_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_bulk_pre_ban_summary: %s\n" % e)
[inline-code-end]