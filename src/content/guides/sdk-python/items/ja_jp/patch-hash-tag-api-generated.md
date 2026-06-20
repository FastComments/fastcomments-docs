## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | はい |  |
| tenantId | string | query | いいえ |  |

## レスポンス

返却: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## 例

[inline-code-attrs-start title = 'patch_hash_tagの例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方式の例を以下に示します。
# ご使用の認証ユースケースに適した例を選んでください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーの接頭辞（例: Bearer）を設定するために下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (オプション)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (オプション)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]