## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| broadcastId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PinComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pin_comment200_response.rb)

## 예제

[inline-code-attrs-start title = 'un_pin_comment 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 문자열 | 
comment_id = 'comment_id_example' # 문자열 | 
broadcast_id = 'broadcast_id_example' # 문자열 | 
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->un_pin_comment: #{e}"
end
[inline-code-end]

---