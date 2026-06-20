## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | упит | Да |  |
| commentId | string | путања | Да |  |
| isFlagged | boolean | упит | Да |  |
| sso | string | упит | Не |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'flag_comment_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
is_flagged = true # Boolean | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->flag_comment_public: #{e}"
end
[inline-code-end]