Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página agora.
Retorna anonCount + totalCount (assinantes da sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Tiebreaker do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identificador da URL da página (limpo no servidor).
opts = {
  after_name: 'after_name_example', # String | Cursor: passe nextAfterName da resposta anterior.
  after_user_id: 'after_user_id_example' # String | Tiebreaker do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName está definido para que empates de nome não descartem entradas.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]