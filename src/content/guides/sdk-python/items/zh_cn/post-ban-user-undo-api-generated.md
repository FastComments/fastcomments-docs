## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## Response

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 示例

[inline-code-attrs-start title = 'post_ban_user_undo 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.ban_user_undo_params import BanUserUndoParams
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.ModerationApi(api_client)
    ban_user_undo_params = client.BanUserUndoParams() # BanUserUndoParams | 
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.post_ban_user_undo(ban_user_undo_params, sso=sso)
        print("The response of ModerationApi->post_ban_user_undo:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_undo: %s\n" % e)
[inline-code-end]