## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |
| minValue | number | query | 否 |  |
| maxValue | number | query | 否 |  |
| limit | number | query | 否 |  |

## 回應

回傳: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_comments_with_question_results200_response.py)

## 範例

[inline-code-attrs-start title = 'combine_comments_with_question_results 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_comments_with_question_results200_response import CombineCommentsWithQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# 指定 host 為可選，預設為 https://fastcomments.com
# 有關所有支援的設定參數，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 並依照 API 伺服器的安全性政策。
# 下方提供每種授權方式的範例，請使用
# 適合您使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列程式來設置 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (可選)
    question_ids = ['question_ids_example'] # List[str] |  (可選)
    url_id = 'url_id_example' # str |  (可選)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (可選)
    force_recalculate = True # bool |  (可選)
    min_value = 3.4 # float |  (可選)
    max_value = 3.4 # float |  (可選)
    limit = 3.4 # float |  (可選)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]