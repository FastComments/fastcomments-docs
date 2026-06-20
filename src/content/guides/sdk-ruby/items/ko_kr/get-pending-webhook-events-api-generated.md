## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니오 |  |
| externalId | string | query | 아니오 |  |
| eventType | string | query | 아니오 |  |
| type | string | query | 아니오 |  |
| domain | string | query | 아니오 |  |
| attemptCountGT | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_pending_webhook_events_response.rb)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_events 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 대한 접두사(예: 'Bearer')를 설정하려면 다음 줄의 주석 처리를 해제하세요 (기본값은 nil)
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
  attempt_count_gt: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_pending_webhook_events(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_pending_webhook_events: #{e}"
end
[inline-code-end]