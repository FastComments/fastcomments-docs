## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_manual_badges_response.rb)

## 예제

[inline-code-attrs-start title = 'get_manual_badges 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_manual_badges(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges: #{e}"
end
[inline-code-end]

---