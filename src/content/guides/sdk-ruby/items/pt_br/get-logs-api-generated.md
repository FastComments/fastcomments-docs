## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_logs_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_logs'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_logs(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Erro ao chamar ModerationApi->get_logs: #{e}"
end
[inline-code-end]