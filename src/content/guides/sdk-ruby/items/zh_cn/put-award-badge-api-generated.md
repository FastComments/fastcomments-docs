## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/award_user_badge_response.rb)

## 示例

[inline-code-attrs-start title = 'put_award_badge 示例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # 字符串 | 
opts = {
  user_id: 'user_id_example', # 字符串 | 
  comment_id: 'comment_id_example', # 字符串 | 
  broadcast_id: 'broadcast_id_example', # 字符串 | 
  sso: 'sso_example' # 字符串 | 
}

begin
  
  result = api_instance.put_award_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_award_badge: #{e}"
end
[inline-code-end]