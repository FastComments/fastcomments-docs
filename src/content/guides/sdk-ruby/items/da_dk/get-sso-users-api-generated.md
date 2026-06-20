## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | integer | query | Nej |  |

## Respons

Returnerer: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_users_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_sso_users Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsætning af autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisation: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren fra følgende linje for at angive et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 56 # Integer | 
}

begin
  
  result = api_instance.get_sso_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_sso_users: #{e}"
end
[inline-code-end]