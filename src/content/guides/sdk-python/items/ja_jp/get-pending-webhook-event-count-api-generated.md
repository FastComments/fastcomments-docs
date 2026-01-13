## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | query | いいえ |  |
| externalId | string | query | いいえ |  |
| eventType | string | query | いいえ |  |
| type | string | query | いいえ |  |
| domain | string | query | いいえ |  |
| attemptCountGT | number | query | いいえ |  |

## レスポンス

戻り値: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count200_response.py)

## 例

[inline-code-attrs-start title = 'get_pending_webhook_event_count の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count200_response import GetPendingWebhookEventCount200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを
# API サーバーのセキュリティポリシーに従って設定する必要があります。
# 各認証方式の例を以下に示します。お使いの認証ユースケースを満たす
# 例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには、下のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  （オプション）
    external_id = 'external_id_example' # str |  （オプション）
    event_type = 'event_type_example' # str |  （オプション）
    type = 'type_example' # str |  （オプション）
    domain = 'domain_example' # str |  （オプション）
    attempt_count_gt = 3.4 # float |  （オプション）

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]