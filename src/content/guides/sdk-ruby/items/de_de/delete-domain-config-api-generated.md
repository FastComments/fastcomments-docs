## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Antwort

Gibt zurück: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/delete_domain_config_response.rb)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für delete_domain_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Autorisierung einrichten
FastCommentsClient.configure do |config|
  # API-Schlüssel-Autorisierung konfigurieren: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain = 'domain_example' # String | 

begin
  
  result = api_instance.delete_domain_config(tenant_id, domain)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_domain_config: #{e}"
end
[inline-code-end]

---