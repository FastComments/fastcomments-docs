## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 範例

[inline-code-attrs-start title = 'delete_question_config 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全性政策設定驗證與授權參數。
# 下方提供各種驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消註解下列設定以為 API 金鑰設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_question_config(tenant_id, id)
        print("The response of DefaultApi->delete_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_question_config: %s\n" % e)
[inline-code-end]