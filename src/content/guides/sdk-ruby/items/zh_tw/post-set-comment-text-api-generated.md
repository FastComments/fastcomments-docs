## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | 路徑 | 是 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/set_comment_text_response.rb)

## 範例

[inline-code-attrs-start title = 'post_set_comment_text 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # 字串 | 
set_comment_text_params = FastCommentsClient::SetCommentTextParams.new({comment: 'comment_example'}) # SetCommentTextParams | 
opts = {
  sso: 'sso_example' # 字串 | 
}

begin
  
  result = api_instance.post_set_comment_text(comment_id, set_comment_text_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_set_comment_text: #{e}"
end
[inline-code-end]