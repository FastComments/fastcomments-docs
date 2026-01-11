## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_notification_count200_response.rb)

## 예제

[inline-code-attrs-start title = 'get_user_notification_count 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_user_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_user_notification_count: #{e}"
end
[inline-code-end]

---