## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |

## 回應

回傳: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes200_response.py)

## 範例

[inline-code-attrs-start title = 'get_votes 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes200_response import GetVotes200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 有關所有支援的設定參數列表，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客戶端必須設定認證與授權參數
# 以符合 API 伺服器的安全策略。
# 下方提供每種認證方法的示例，請使用
# 適合您使用情境的示例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消註解以下內容以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]