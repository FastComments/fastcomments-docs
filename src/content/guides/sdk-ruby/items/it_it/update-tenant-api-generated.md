## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_tenant'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# impostazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione tramite API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Rimuovi il commento dalla riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_tenant_body = FastCommentsClient::UpdateTenantBody.new # UpdateTenantBody | 

begin
  
  result = api_instance.update_tenant(tenant_id, id, update_tenant_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_tenant: #{e}"
end
[inline-code-end]