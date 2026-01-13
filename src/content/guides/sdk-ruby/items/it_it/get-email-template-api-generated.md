## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`GetEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template200_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_email_template'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configura l'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione con API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_email_template(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template: #{e}"
end
[inline-code-end]

---