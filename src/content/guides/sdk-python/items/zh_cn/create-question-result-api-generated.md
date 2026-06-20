## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 响应

返回: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result_response.py)

## 示例

[inline-code-attrs-start title = 'create_question_result 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result_body import CreateQuestionResultBody
from client.models.create_question_result_response import CreateQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以符合 API 服务器的安全策略。
# 下方提供了每种认证方法的示例，请使用
# 能满足您认证场景的示例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_result_body = client.CreateQuestionResultBody() # CreateQuestionResultBody | 

    try:
        api_response = api_instance.create_question_result(tenant_id, create_question_result_body)
        print("The response of DefaultApi->create_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_result: %s\n" % e)
[inline-code-end]