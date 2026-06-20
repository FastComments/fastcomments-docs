## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| badgeId | string | query | はい |  |
| userId | string | query | いいえ |  |
| commentId | string | query | いいえ |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/remove_user_badge_response.rb)

## 例

[inline-code-attrs-start title = 'put_remove_badge の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
badge_id = 'badge_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  comment_id: 'comment_id_example', # String | 
  broadcast_id: 'broadcast_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.put_remove_badge(badge_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->put_remove_badge: #{e}"
end
[inline-code-end]