## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | път | Да |  |
| commentId | string | път | Да |  |
| broadcastId | string | заявка | Да |  |
| sso | string | заявка | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'lock_comment Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Стринг | 
comment_id = 'comment_id_example' # Стринг | 
broadcast_id = 'broadcast_id_example' # Стринг | 
opts = {
  sso: 'sso_example' # Стринг | 
}

begin
  
  result = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->lock_comment: #{e}"
end
[inline-code-end]