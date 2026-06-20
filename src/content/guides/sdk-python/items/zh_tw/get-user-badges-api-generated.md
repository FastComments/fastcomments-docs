## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## 回應

回傳：[`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_badges 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供了各種驗證方法的範例，請使用
# 符合您驗證使用情境的範例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 透過 API 用戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (選用)
    badge_id = 'badge_id_example' # str |  (選用)
    type = 3.4 # float |  (選用)
    displayed_on_comments = True # bool |  (選用)
    limit = 3.4 # float |  (選用)
    skip = 3.4 # float |  (選用)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]