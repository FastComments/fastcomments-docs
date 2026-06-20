## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| contextUserId | string | query | 否 |  |
| isLive | boolean | query | 否 |  |

## 回傳

回傳: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## 範例

[inline-code-attrs-start title = 'delete_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援設定參數的清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全性政策配置認證與授權參數。
# 下面提供了每種驗證方法的範例，請使用符合您驗證使用情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 用戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (選用)
    is_live = True # bool |  (選用)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]