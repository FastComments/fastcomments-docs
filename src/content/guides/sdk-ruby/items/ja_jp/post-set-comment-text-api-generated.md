## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_comment_text_response.rb)

## 例

[inline-code-attrs-start title = 'post_set_comment_text の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 文字列 | 
set_comment_text_params = FastCommentsClient::SetCommentTextParams.new({comment: 'comment_example'}) # SetCommentTextParams | 
opts = {
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.post_set_comment_text(comment_id, set_comment_text_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_text: #{e}"
end
[inline-code-end]

---