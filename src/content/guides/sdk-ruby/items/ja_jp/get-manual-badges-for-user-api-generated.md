## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| badgesUserId | string | クエリ | いいえ |  |
| commentId | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

返却値: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_manual_badges_response.rb)

## 例

[inline-code-attrs-start title = 'get_manual_badges_for_user の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  badges_user_id: 'badges_user_id_example', # String | 
  comment_id: 'comment_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_manual_badges_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges_for_user: #{e}"
end
[inline-code-end]