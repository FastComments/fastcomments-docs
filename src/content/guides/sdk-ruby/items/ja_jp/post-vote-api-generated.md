## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## 例

[inline-code-attrs-start title = 'post_vote の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  direction: 'direction_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_vote(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_vote: #{e}"
end
[inline-code-end]

---