## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_trust_factor_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_trust_factor'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  user_id: 'user_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_trust_factor(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_trust_factor: #{e}"
end
[inline-code-end]

---