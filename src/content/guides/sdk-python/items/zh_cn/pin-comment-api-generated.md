## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| broadcastId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回： [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_comment_pin_status_response.py)

## 示例

[inline-code-attrs-start title = 'pin_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_comment_pin_status_response import ChangeCommentPinStatusResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  （可选）

    try:
        api_response = api_instance.pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->pin_comment: %s\n" % e)
[inline-code-end]

---