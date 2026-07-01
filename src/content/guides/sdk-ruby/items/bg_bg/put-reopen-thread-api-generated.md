## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Пример

[inline-code-attrs-start title = 'put_reopen_thread Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Низ | 
url_id = 'url_id_example' # Низ | 
opts = {
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.put_reopen_thread(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Грешка при извикване на ModerationApi->put_reopen_thread: #{e}"
end
[inline-code-end]