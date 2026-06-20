## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 回應

回傳: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_result_response.py)

## 範例

[inline-code-attrs-start title = 'get_question_result 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_result_response import GetQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以查看所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全性政策。
# 下方提供每種認證方法的範例，請使用符合您認證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下面以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API 客戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的一個實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_result(tenant_id, id)
        print("The response of DefaultApi->get_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_result: %s\n" % e)
[inline-code-end]