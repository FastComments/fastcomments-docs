## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notifications200_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_notifications Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notifications200_response import GetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 進入包含 API 用戶端實例的上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page_size = 56 # int |  (選填)
    after_id = 'after_id_example' # str |  (選填)
    include_context = True # bool |  (選填)
    after_created_at = 56 # int |  (選填)
    unread_only = True # bool |  (選填)
    dm_only = True # bool |  (選填)
    no_dm = True # bool |  (選填)
    include_translations = True # bool |  (選填)
    sso = 'sso_example' # str |  (選填)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]