---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_page_api_response.py)

## 範例

[inline-code-attrs-start title = 'patch_page 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_page_api_response import PatchPageAPIResponse
from client.models.update_api_page_data import UpdateAPIPageData
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全性政策。
# 下方提供每種驗證方法的範例，請使用
# 符合您驗證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下面那行以為 API 金鑰設定前綴 (例如 Bearer)
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的一個實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_page_data = client.UpdateAPIPageData() # UpdateAPIPageData | 

    try:
        api_response = api_instance.patch_page(tenant_id, id, update_api_page_data)
        print("The response of DefaultApi->patch_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_page: %s\n" % e)
[inline-code-end]

---