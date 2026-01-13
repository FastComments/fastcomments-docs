## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|---------|---------|--------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Retourneert: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_package200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_tenant_package Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaarregel hieronder weg om een prefix voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard is nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_tenant_package(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_package: #{e}"
end
[inline-code-end]

---