## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_users200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_tenant_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autenticazione tramite API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tenant_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_users: #{e}"
end
[inline-code-end]

---