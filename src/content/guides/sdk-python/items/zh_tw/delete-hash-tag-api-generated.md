## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Yes |  |
| tenantId | string | query | No |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'delete_hash_tag 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.delete_hash_tag_request_body import DeleteHashTagRequestBody
from client.rest import ApiException
from pprint import pprint

# 設定 host 為可選，預設為 https://fastcomments.com
# 請參見 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全性政策設定驗證與授權參數。
# 下方提供每種授權方法的範例，請使用符合您驗證情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下方程式以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str | (選用)
    delete_hash_tag_request_body = client.DeleteHashTagRequestBody() # DeleteHashTagRequestBody | (選用)

    try:
        api_response = api_instance.delete_hash_tag(tag, tenant_id=tenant_id, delete_hash_tag_request_body=delete_hash_tag_request_body)
        print("The response of DefaultApi->delete_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_hash_tag: %s\n" % e)
[inline-code-end]