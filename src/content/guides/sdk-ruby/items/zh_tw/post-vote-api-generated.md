## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## 範例

[inline-code-attrs-start title = 'post_vote 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 字串 | 
opts = {
  direction: 'direction_example', # 字串 | 
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.post_vote(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_vote: #{e}"
end
[inline-code-end]

---