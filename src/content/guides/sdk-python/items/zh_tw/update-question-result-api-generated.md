## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 範例

[inline-code-attrs-start title = 'update_question_result 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_question_result_body import UpdateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# 定義 host 為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
# 用戶端必須根據 API 伺服器的安全性政策設定驗證和授權參數。
# 下方提供每種驗證方法的範例，
# 請使用符合您驗證需求的範例。

# 設定 API 金鑰授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列內容以設定前綴 (e.g. Bearer) 用於 API 金鑰
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