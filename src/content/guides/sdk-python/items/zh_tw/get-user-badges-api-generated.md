## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| userId | string | 查詢 | 否 |  |
| badgeId | string | 查詢 | 否 |  |
| type | number | 查詢 | 否 |  |
| displayedOnComments | boolean | 查詢 | 否 |  |
| limit | number | 查詢 | 否 |  |
| skip | number | 查詢 | 否 |  |

## 回應

Returns: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_badges 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgesOptions
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以獲得所有受支援設定參數的清單。
# 客戶端必須依據 API 伺服器的安全政策配置驗證與授權參數。
# 為每種驗證方法提供以下範例，使用符合您驗證使用情境的範例。

# 配置 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解以下行以設定 API 金鑰的前綴（例如 Bearer）{
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例建立上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    user_id = 'user_id_example' # str |  （可選）
    badge_id = 'badge_id_example' # str |  （可選）
    type = 3.4 # float |  （可選）
    displayed_on_comments = True # bool |  （可選）
    limit = 3.4 # float |  （可選）
    skip = 3.4 # float |  （可選）

    try:
        api_response = api_instance.get_user_badges(tenant_id, GetUserBadgesOptions(user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip))
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]

---