## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| trustFactor | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## 範例

[inline-code-attrs-start title = 'set_trust_factor 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 以查看所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 用戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (可選)
    trust_factor = 'trust_factor_example' # str |  (可選)
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]