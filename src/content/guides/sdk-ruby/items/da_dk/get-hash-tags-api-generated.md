## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nej |  |

## Svar

Returnerer: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_hash_tags200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_hash_tags Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt godkendelse
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisering: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren fra følgende linje for at sætte et præfiks til API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_hash_tags(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_hash_tags: #{e}"
end
[inline-code-end]