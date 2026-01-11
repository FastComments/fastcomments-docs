## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 回應

回傳: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_subscription_api_response.py)

## 範例

[inline-code-attrs-start title = 'create_subscription 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_api_user_subscription_data import CreateAPIUserSubscriptionData
from client.models.create_subscription_api_response import CreateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 有關所有支援的設定參數清單，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全性政策來設定驗證與授權參數。
# 下方提供每種驗證方法的範例，請使用符合您驗證需求的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列程式碼以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在有 API 客戶端實例的情境中執行
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_user_subscription_data = client.CreateAPIUserSubscriptionData() # CreateAPIUserSubscriptionData | 

    try:
        api_response = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
        print("The response of DefaultApi->create_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_subscription: %s\n" % e)
[inline-code-end]