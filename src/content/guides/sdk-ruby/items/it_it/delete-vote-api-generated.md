## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| editKey | string | query | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_comment_vote200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione tramite chiave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Rimuovere il commento dalla riga seguente per impostare un prefisso per la chiave API, es. 'Bearer' (valore predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  edit_key: 'edit_key_example' # String | 
}

begin
  
  result = api_instance.delete_vote(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_vote: #{e}"
end
[inline-code-end]