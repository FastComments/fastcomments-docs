## Parameters

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## Response

戻り値: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## 例

[inline-code-attrs-start title = 'add_hash_tag 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py を参照してください。サポートされているすべての設定パラメータの一覧があります。
# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従って。
# 各認証方式の例が以下に提供されています。使用する例を選んでください
# あなたの認証ユースケースに適合するものです。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、以下のコメントを解除して API キーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]