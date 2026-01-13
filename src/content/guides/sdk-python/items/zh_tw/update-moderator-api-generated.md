## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| id | string | 路徑 | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 範例

[inline-code-attrs-start title = 'update_moderator 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_moderator_body import UpdateModeratorBody
from client.rest import ApiException
from pprint import pprint

# 定義主機為可選，預設值為 https://fastcomments.com
# 請參閱 configuration.py，了解所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種授權方法的範例，請使用
# 適用於您授權使用情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 的實例建立一個上下文環境
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_moderator_body = client.UpdateModeratorBody() # UpdateModeratorBody | 

    try:
        api_response = api_instance.update_moderator(tenant_id, id, update_moderator_body)
        print("The response of DefaultApi->update_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_moderator: %s\n" % e)
[inline-code-end]