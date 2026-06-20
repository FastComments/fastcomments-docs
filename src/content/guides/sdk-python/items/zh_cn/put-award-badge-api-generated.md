---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## 示例

[inline-code-attrs-start title = 'put_award_badge 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (可选)
    comment_id = 'comment_id_example' # str |  (可选)
    broadcast_id = 'broadcast_id_example' # str |  (可选)
    sso = 'sso_example' # str |  (可选)

    try:
        api_response = api_instance.put_award_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]

---