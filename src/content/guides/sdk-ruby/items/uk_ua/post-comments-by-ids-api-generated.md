## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_api_child_comments_response.rb)

## Приклад

[inline-code-attrs-start title = 'post_comments_by_ids Приклад'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  puts "Помилка під час виклику ModerationApi->post_comments_by_ids: #{e}"
end
[inline-code-end]