## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| domain | string | path | はい |  |

## レスポンス

返却値: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config_response.py)

## 例

[inline-code-attrs-start title = 'delete_domain_config の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config_response import DeleteDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方式の例は以下に示されています、使用する例は
# あなたの認証ユースケースに合致するものを選んでください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]