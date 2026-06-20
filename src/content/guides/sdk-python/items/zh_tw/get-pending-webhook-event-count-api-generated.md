## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 否 |  |
| externalId | string | query | 否 |  |
| eventType | string | query | 否 |  |
| type | string | query | 否 |  |
| domain | string | query | 否 |  |
| attemptCountGT | number | query | 否 |  |

## 回應

回傳: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## 範例

[inline-code-attrs-start title = 'get_pending_webhook_event_count 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全性策略。
# 下方提供了每種驗證方法的範例，
# 請使用符合您驗證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (選用)
    external_id = 'external_id_example' # str |  (選用)
    event_type = 'event_type_example' # str |  (選用)
    type = 'type_example' # str |  (選用)
    domain = 'domain_example' # str |  (選用)
    attempt_count_gt = 3.4 # float |  (選用)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]