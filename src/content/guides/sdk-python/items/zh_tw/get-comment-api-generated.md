## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comment_response.py)

## 範例

[inline-code-attrs-start title = 'get_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comment_response import APIGetCommentResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供了每種認證方法的範例，請使用
# 最符合您認證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 用戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]