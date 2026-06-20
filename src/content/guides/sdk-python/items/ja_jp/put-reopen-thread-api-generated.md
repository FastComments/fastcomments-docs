---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| urlId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 例

[inline-code-attrs-start title = 'put_reopen_thread の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# ホストの設定は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを生成します
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str |  （オプション）

    try:
        api_response = api_instance.put_reopen_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_reopen_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_reopen_thread: %s\n" % e)
[inline-code-end]

---