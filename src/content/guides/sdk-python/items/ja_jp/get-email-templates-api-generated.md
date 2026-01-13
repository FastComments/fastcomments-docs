## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates200_response.py)

## 例

[inline-code-attrs-start title = 'get_email_templates の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates200_response import GetEmailTemplates200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバのセキュリティポリシーに従って、認証および認可のパラメータを設定する必要があります。
# 各認証方法の例を以下に示します。ご自身の認証ユースケースに合った例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには、以下のコメントアウトを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  （オプション）

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]