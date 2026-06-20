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
| skip | number | query | 否 |  |

## 回應

Returns: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## 範例

[inline-code-attrs-start title = 'get_pending_webhook_events 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全性政策來設定驗證與授權參數。
# 下方提供了每種驗證方法的範例，請使用符合您驗證情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列以為 API key 設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optional)
    external_id = 'external_id_example' # str |  (optional)
    event_type = 'event_type_example' # str |  (optional)
    type = 'type_example' # str |  (optional)
    domain = 'domain_example' # str |  (optional)
    attempt_count_gt = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]