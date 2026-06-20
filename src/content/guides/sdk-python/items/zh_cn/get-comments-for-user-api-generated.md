## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| direction | string | query | 否 |  |
| repliesToUserId | string | query | 否 |  |
| page | number | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |

## 响应

返回：[`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## 示例

[inline-code-attrs-start title = 'get_comments_for_user 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (可选)
    direction = client.SortDirections() # SortDirections |  (可选)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (可选)
    page = 3.4 # float |  (可选)
    includei10n = True # bool |  (可选)
    locale = 'locale_example' # str |  (可选)
    is_crawler = True # bool |  (可选)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]