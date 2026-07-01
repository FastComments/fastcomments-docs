## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_child_comments_response.rb)

## 예시

[inline-code-attrs-start title = 'post_comments_by_ids 예시'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comments_by_ids_params = FastCommentsClient::CommentsByIdsParams.new({ids: ['ids_example']}) # CommentsByIdsParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_comments_by_ids(tenant_id, comments_by_ids_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "ModerationApi->post_comments_by_ids 호출 시 오류 발생: #{e}"
end
[inline-code-end]