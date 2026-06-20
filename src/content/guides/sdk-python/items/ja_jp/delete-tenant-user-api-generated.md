## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | string | query | No |  |
| commentDeleteMode | string | query | No |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 例

[inline-code-attrs-start title = 'delete_tenant_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、省略した場合は https://fastcomments.com がデフォルトです
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方式の例を以下に示します。自分のユースケースに合う例を使ってください。

# API キー認証の設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーにプレフィックス（例: Bearer）を設定する場合は、下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いてコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (オプション)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (オプション)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]