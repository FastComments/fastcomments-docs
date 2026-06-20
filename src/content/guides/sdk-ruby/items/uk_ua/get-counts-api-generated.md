## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_banned_users_count_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_counts'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.get_counts(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_counts: #{e}"
end
[inline-code-end]

---