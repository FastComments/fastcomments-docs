## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 | 用於判斷當前頁面是否已訂閱。 |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| includeTenantNotifications | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_notifications 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為可選，預設值為 https://fastcomments.com
# 請參閱 configuration.py 了解所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 用於判斷當前頁面是否已訂閱。 (選填)
    page_size = 56 # int |  (選填)
    after_id = 'after_id_example' # str |  (選填)
    include_context = True # bool |  (選填)
    after_created_at = 56 # int |  (選填)
    unread_only = True # bool |  (選填)
    dm_only = True # bool |  (選填)
    no_dm = True # bool |  (選填)
    include_translations = True # bool |  (選填)
    include_tenant_notifications = True # bool |  (選填)
    sso = 'sso_example' # str |  (選填)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]