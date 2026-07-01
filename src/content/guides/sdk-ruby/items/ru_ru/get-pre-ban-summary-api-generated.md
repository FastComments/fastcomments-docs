## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|--------------|--------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| includeByUserIdAndEmail | boolean | query | Нет |  |
| includeByIP | boolean | query | Нет |  |
| includeByEmailDomain | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## Пример

[inline-code-attrs-start title = 'get_pre_ban_summary Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Строка | 
comment_id = 'comment_id_example' # Строка | 
opts = {
  include_by_user_id_and_email: true, # Булево | 
  include_by_ip: true, # Булево | 
  include_by_email_domain: true, # Булево | 
  sso: 'sso_example' # Строка | 
}

begin
  
  result = api_instance.get_pre_ban_summary(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Ошибка при вызове ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]

---