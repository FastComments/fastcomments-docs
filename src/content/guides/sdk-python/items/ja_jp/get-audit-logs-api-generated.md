## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| order | string | query | いいえ |  |
| after | number | query | いいえ |  |
| before | number | query | いいえ |  |

## レスポンス

戻り値: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## 例

[inline-code-attrs-start title = 'get_audit_logs の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを
# API サーバーのセキュリティポリシーに従って設定する必要があります。
# 各認証方式の例を以下に示します。
# ご自身のユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キー用の接頭辞（例: Bearer）を設定するには、以下のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使用してコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)
    order = client.SORTDIR() # SORTDIR |  (optional)
    after = 3.4 # float |  (optional)
    before = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]