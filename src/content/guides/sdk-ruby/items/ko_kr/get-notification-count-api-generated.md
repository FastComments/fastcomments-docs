## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| fromCommentId | string | query | 아니요 |  |
| viewed | boolean | query | 아니요 |  |
| type | string | query | 아니요 |  |

## 응답

반환: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notification_count200_response.rb)

## 예제

[inline-code-attrs-start title = 'get_notification_count 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사를 설정하려면 다음 줄의 주석을 제거하세요. 예: 'Bearer' (기본값: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
opts = {
  user_id: 'user_id_example', # 문자열 | 
  url_id: 'url_id_example', # 문자열 | 
  from_comment_id: 'from_comment_id_example', # 문자열 | 
  viewed: true, # 부울 | 
  type: 'type_example' # 문자열 | 
}

begin
  
  result = api_instance.get_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notification_count: #{e}"
end
[inline-code-end]