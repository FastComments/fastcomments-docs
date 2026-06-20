## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | 用于确定当前页面是否已订阅。 |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## 响应

返回: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## 示例

[inline-code-attrs-start title = 'get_user_notifications 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有受支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 用于确定当前页面是否已订阅。 (可选)
    page_size = 56 # int |  (可选)
    after_id = 'after_id_example' # str |  (可选)
    include_context = True # bool |  (可选)
    after_created_at = 56 # int |  (可选)
    unread_only = True # bool |  (可选)
    dm_only = True # bool |  (可选)
    no_dm = True # bool |  (可选)
    include_translations = True # bool |  (可选)
    include_tenant_notifications = True # bool |  (可选)
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]