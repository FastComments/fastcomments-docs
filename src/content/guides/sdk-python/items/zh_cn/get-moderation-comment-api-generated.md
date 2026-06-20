## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| includeEmail | boolean | query | No |  |
| includeIP | boolean | query | No |  |
| sso | string | query | No |  |

## 响应

返回：[`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_comment_response.py)

## 示例

[inline-code-attrs-start title = 'get_moderation_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_comment_response import ModerationAPICommentResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_email = True # bool |  (可选)
    include_ip = True # bool |  (可选)
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.get_moderation_comment(comment_id, include_email=include_email, include_ip=include_ip, sso=sso)
        print("The response of ModerationApi->get_moderation_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment: %s\n" % e)
[inline-code-end]

---