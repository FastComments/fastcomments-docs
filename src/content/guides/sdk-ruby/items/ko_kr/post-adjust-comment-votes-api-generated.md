## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/adjust_votes_response.rb)

## 예제

[inline-code-attrs-start title = 'post_adjust_comment_votes 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 문자열 | 
adjust_comment_votes_params = FastCommentsClient::AdjustCommentVotesParams.new({adjust_vote_amount: 3.56}) # AdjustCommentVotesParams | 
opts = {
  sso: 'sso_example' # 문자열 | 
}

begin
  
  result = api_instance.post_adjust_comment_votes(comment_id, adjust_comment_votes_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_adjust_comment_votes: #{e}"
end
[inline-code-end]