## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## 回應

回傳: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_event_count200_response.rb)

## 範例

[inline-code-attrs-start title = 'get_pending_webhook_event_count 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 設定授權
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # 取消註解下列行以為 API 金鑰設定前綴，例如 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  comment_id: 'comment_id_example', # String | 
  external_id: 'external_id_example', # String | 
  event_type: 'event_type_example', # String | 
  type: 'type_example', # String | 
  domain: 'domain_example', # String | 
  attempt_count_gt: 1.2 # Float | 
}

begin
  
  result = api_instance.get_pending_webhook_event_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_pending_webhook_event_count: #{e}"
end
[inline-code-end]

---