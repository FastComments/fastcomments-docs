## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Response

返回：[`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## 範例

[inline-code-attrs-start title = 'delete_sso_user 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 了解所有支援的設定參數清單。
# 客戶端必須依照 API 伺服器的安全策略設定驗證與授權參數。
# 以下提供每種驗證方法的範例，使用符合您驗證需求的範例。
# 滿足您的驗證使用情境。

# 設定 API 金鑰授權：api_key
# 取消註解以下以設定 API 金鑰的前綴（例如 Bearer），如有需要
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 進入包含 API 客戶端實例的情境
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (optional)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (optional)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]