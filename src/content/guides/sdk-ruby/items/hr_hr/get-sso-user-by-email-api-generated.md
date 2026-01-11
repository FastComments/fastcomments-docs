## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| email | string | path | Da |  |

## Odgovor

Vraća: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_user_by_email_a_p_i_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_sso_user_by_email Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurirajte autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentirajte sljedeći redak da postavite prefiks za API ključ, npr. 'Bearer' (zadano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
email = 'email_example' # String | 

begin
  
  result = api_instance.get_sso_user_by_email(tenant_id, email)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_sso_user_by_email: #{e}"
end
[inline-code-end]