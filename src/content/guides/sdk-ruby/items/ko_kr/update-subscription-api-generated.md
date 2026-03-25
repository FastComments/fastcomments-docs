## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| userId | string | query | 아니오 |  |

## 응답

반환: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_subscription_a_p_i_response.rb)

## 예제

[inline-code-attrs-start title = 'update_subscription 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# 인증 설정
FastCommentsClient.configure do |config|
  # API 키 인증 구성: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API 키에 접두사를 설정하려면 다음 줄의 주석을 제거하세요. 예: 'Bearer' (기본값은 nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_user_subscription_data = FastCommentsClient::UpdateAPIUserSubscriptionData.new # UpdateAPIUserSubscriptionData | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_subscription: #{e}"
end
[inline-code-end]

---