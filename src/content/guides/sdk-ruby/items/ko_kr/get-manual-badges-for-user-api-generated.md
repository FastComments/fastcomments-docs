## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | 아니요 |  |
| commentId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_manual_badges_response.rb)

## 예제

[inline-code-attrs-start title = 'get_manual_badges_for_user 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  badges_user_id: 'badges_user_id_example', # 문자열 | 
  comment_id: 'comment_id_example', # 문자열 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_manual_badges_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges_for_user: #{e}"
end
[inline-code-end]

---