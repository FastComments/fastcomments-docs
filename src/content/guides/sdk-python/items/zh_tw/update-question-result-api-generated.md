## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回傳

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'update_question_result 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_question_result_body import UpdateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 有關所有支援的設定參數清單，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全性政策來設定驗證與授權參數。
# 每種驗證方法的範例在下方提供，請使用符合您驗證情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，取消註解以下以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_result_body = client.UpdateQuestionResultBody() # UpdateQuestionResultBody | 

    try:
        api_response = api_instance.update_question_result(tenant_id, id, update_question_result_body)
        print("The response of DefaultApi->update_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_result: %s\n" % e)
[inline-code-end]