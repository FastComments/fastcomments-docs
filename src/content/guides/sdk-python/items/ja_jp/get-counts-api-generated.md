## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| sso | string | query | 任意 |  |

## レスポンス

戻り値: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_count_response.py)

## 例

[inline-code-attrs-start title = 'get_counts の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_count_response import GetBannedUsersCountResponse
from client.rest import ApiException
from pprint import pprint

# ホストの設定は任意で、デフォルトは https://fastcomments.com です
# すべてのサポートされている設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (任意)

    try:
        api_response = api_instance.get_counts(sso=sso)
        print("The response of ModerationApi->get_counts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_counts: %s\n" % e)
[inline-code-end]

---