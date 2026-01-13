## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## レスポンス

戻り値: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_configs200_response.py)

## 例

[inline-code-attrs-start title = 'get_domain_configs の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_configs200_response import GetDomainConfigs200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを
# API サーバーのセキュリティポリシーに従って設定する必要があります。
# 各認証方式の例は下に示しています。あなたの認証ユースケースに合った例を使用してください。

# Configure API key authorization: api_key
# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには下のコメントを外してください

# Enter a context with an instance of the API client
# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_domain_configs(tenant_id)
        print("The response of DefaultApi->get_domain_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_configs: %s\n" % e)
[inline-code-end]