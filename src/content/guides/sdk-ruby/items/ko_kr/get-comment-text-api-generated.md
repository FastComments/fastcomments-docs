## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| editKey | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/public_a_p_i_get_comment_text_response.rb)

## 예제

[inline-code-attrs-start title = 'get_comment_text 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  edit_key: 'edit_key_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_comment_text(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comment_text: #{e}"
end
[inline-code-end]