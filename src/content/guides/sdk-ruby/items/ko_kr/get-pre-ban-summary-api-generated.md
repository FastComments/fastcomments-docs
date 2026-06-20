## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| includeByUserIdAndEmail | boolean | query | 아니오 |  |
| includeByIP | boolean | query | 아니오 |  |
| includeByEmailDomain | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## 예제

[inline-code-attrs-start title = 'get_pre_ban_summary 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 문자열 | 
opts = {
  include_by_user_id_and_email: true, # 불리언 | 
  include_by_ip: true, # 불리언 | 
  include_by_email_domain: true, # 불리언 | 
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.get_pre_ban_summary(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]