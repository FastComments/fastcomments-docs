## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | integer | query | Ne |  |

## Odgovor

Vraća: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_users_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_sso_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# podešavanje autorizacije
FastCommentsClient.configure do |config|
  # Konfiguriši autorizaciju pomoću API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentariši sledeći red da postaviš prefiks za API ključ, npr. 'Bearer' (podrazumevano nil)
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