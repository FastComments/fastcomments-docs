## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| badgeId | string | query | 否 |  |
| type | number | query | 否 |  |
| displayedOnComments | boolean | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badges200_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_badges 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badges200_response import GetUserBadges200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全性政策設定認證與授權參數。
# 下方提供每種授權方法的範例，請使用符合您授權需求的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (可選)
    badge_id = 'badge_id_example' # str |  (可選)
    type = 3.4 # float |  (可選)
    displayed_on_comments = True # bool |  (可選)
    limit = 3.4 # float |  (可選)
    skip = 3.4 # float |  (可選)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]