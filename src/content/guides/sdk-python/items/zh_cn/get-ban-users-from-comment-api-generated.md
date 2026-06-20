## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | 路径 | 是 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_from_comment_response.py)

## 示例

[inline-code-attrs-start title = 'get_ban_users_from_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_from_comment_response import GetBannedUsersFromCommentResponse
from client.rest import ApiException
from pprint import pprint

# 指定主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.get_ban_users_from_comment(comment_id, sso=sso)
        print("The response of ModerationApi->get_ban_users_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_ban_users_from_comment: %s\n" % e)
[inline-code-end]