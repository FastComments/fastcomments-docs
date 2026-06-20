## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | 否 |  |
| count | number | query | 否 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## 示例

[inline-code-attrs-start title = 'get_api_comments 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参见 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  （可选）
    count = 3.4 # float |  （可选）
    text_search = 'text_search_example' # str |  （可选）
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  （可选）
    filters = 'filters_example' # str |  （可选）
    search_filters = 'search_filters_example' # str |  （可选）
    sorts = 'sorts_example' # str |  （可选）
    demo = True # bool |  （可选）
    sso = 'sso_example' # str |  （可选）

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]