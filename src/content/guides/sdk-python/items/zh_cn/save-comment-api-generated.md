## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| sendEmails | boolean | query | 否 |  |
| populateNotifications | boolean | query | 否 |  |

## 响应

返回: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comment200_response.py)

## 示例

[inline-code-attrs-start title = 'save_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comment200_response import SaveComment200Response
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下方提供了每种认证方法的示例，请使用适合您认证用例的示例。
#
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消下面的注释以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (可选)
    do_spam_check = True # bool |  (可选)
    send_emails = True # bool |  (可选)
    populate_notifications = True # bool |  (可选)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]

---