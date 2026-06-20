---
## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| sso | string | query | 아니오 |  |

## 응답

반환값: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## 예제

[inline-code-attrs-start title = 'post_ban_user_undo 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
ban_user_undo_params = FastCommentsClient::BanUserUndoParams.new({changelog: FastCommentsClient::APIBanUserChangeLog.new}) # BanUserUndoParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_ban_user_undo(ban_user_undo_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_undo: #{e}"
end
[inline-code-end]

---