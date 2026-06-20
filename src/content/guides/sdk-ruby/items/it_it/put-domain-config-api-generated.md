## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| domainToUpdate | string | path | Sì |  |

## Risposta

Restituisce: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/put_domain_config_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di put_domain_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Rimuovere il commento dalla riga seguente per impostare un prefisso per la chiave API, es. 'Bearer' (predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain_to_update = 'domain_to_update_example' # String | 
update_domain_config_params = FastCommentsClient::UpdateDomainConfigParams.new({domain: 'domain_example'}) # UpdateDomainConfigParams | 

begin
  
  result = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->put_domain_config: #{e}"
end
[inline-code-end]