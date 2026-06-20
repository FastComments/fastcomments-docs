## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| value | string | query | 任意 |  |
| sso | string | query | 任意 |  |

## レスポンス

戻り値: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_user_search_response.py)

## 例

[inline-code-attrs-start title = 'get_search_users の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_user_search_response import ModerationUserSearchResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str | (オプション)
    sso = 'sso_example' # str | (オプション)

    try:
        api_response = api_instance.get_search_users(value=value, sso=sso)
        print("The response of ModerationApi->get_search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_users: %s\n" % e)
[inline-code-end]