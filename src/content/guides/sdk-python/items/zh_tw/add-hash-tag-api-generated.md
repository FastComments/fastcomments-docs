## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 否 |  |

## 回應

回傳: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## 範例

[inline-code-attrs-start title = 'add_hash_tag 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選擇性，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全性政策設定驗證與授權參數。
# 下方提供了各種驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API key 驗證：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解以下以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (optional)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]

---