## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| afterId | string | 查询 | 否 |  |
| afterCreatedAt | integer | 查询 | 否 |  |
| unreadOnly | boolean | 查询 | 否 |  |
| dmOnly | boolean | 查询 | 否 |  |
| noDm | boolean | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications200_response.py)

## 示例

[inline-code-attrs-start title = 'reset_user_notifications 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications200_response import ResetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (可选)
    after_created_at = 56 # int |  (可选)
    unread_only = True # bool |  (可选)
    dm_only = True # bool |  (可选)
    no_dm = True # bool |  (可选)
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso)
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]