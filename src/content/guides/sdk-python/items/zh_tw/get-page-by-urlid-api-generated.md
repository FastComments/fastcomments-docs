## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |

## 回應

回傳: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_page_by_urlid_api_response.py)

## 範例

[inline-code-attrs-start title = 'get_page_by_urlid 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_page_by_urlid_api_response import GetPageByURLIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以獲取所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全政策設定驗證與授權參數。
# 以下提供每種認證方式的範例，請使用符合您使用情境的範例。
#
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消下方註解以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_page_by_urlid(tenant_id, url_id)
        print("The response of DefaultApi->get_page_by_urlid:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_page_by_urlid: %s\n" % e)
[inline-code-end]

---