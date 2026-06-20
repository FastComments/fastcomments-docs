## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## 例

[inline-code-attrs-start title = 'add_hash_tag の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例を以下に示します。自分のユースケースに合った例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下の行のコメントアウトを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスのコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | （任意）
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody | （任意）

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]