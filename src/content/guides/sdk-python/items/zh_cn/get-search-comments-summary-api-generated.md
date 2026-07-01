## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| value | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sso | string | query | No |  |

## 响应

返回: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## 示例

[inline-code-attrs-start title = 'get_search_comments_summary 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchCommentsSummaryOptions
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
# 创建 API 类的实例
api_instance = client.ModerationApi(api_client)
tenant_id = 'tenant_id_example' # str | 
value = 'value_example' # str |  （可选）
filters = 'filters_example' # str |  （可选）
search_filters = 'search_filters_example' # str |  （可选）
sso = 'sso_example' # str |  （可选）

try:
    api_response = api_instance.get_search_comments_summary(tenant_id, GetSearchCommentsSummaryOptions(value=value, filters=filters, search_filters=search_filters, sso=sso))
    print("The response of ModerationApi->get_search_comments_summary:\n")
    pprint(api_response)
except Exception as e:
    print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]

---