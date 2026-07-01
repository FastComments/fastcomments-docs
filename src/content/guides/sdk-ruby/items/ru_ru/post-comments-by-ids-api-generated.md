## Параметры

| Имя | Тип | Размещение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_child_comments_response.rb)

## Пример

[inline-code-attrs-start title = 'post_comments_by_ids Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  puts "Ошибка при вызове ModerationApi->post_comments_by_ids: #{e}"
end
[inline-code-end]