## パラメータ

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## レスポンス

返却値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## 例

[inline-code-attrs-start title = 'add_hash_tags_bulk の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従ってください。
# 以下に各認証方式の例を示します。
# ご自身の認証ユースケースに合う例を使用してください。

# APIキー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには下記のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (任意)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (任意)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]