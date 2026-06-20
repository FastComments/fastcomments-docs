## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## 示例

[inline-code-attrs-start title = 'get_question_results 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# 指定主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下方提供了每种认证方法的示例，请使用符合您认证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在包含 API 客户端实例的上下文中执行
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (可选)
    user_id = 'user_id_example' # str |  (可选)
    start_date = 'start_date_example' # str |  (可选)
    question_id = 'question_id_example' # str |  (可选)
    question_ids = 'question_ids_example' # str |  (可选)
    skip = 3.4 # float |  (可选)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]