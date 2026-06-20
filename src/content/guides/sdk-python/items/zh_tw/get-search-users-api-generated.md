## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_user_search_response.py)

## 範例

[inline-code-attrs-start title = 'get_search_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_user_search_response import ModerationUserSearchResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選項，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個 context
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (選填)
    sso = 'sso_example' # str |  (選填)

    try:
        api_response = api_instance.get_search_users(value=value, sso=sso)
        print("The response of ModerationApi->get_search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_users: %s\n" % e)
[inline-code-end]