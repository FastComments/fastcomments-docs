## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| sso | string | query | Não |  |

## Resposta

Retorna: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_moderate_get_user_ban_preferences_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_ban_preference'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_ban_preference(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_user_ban_preference: #{e}"
end
[inline-code-end]

---