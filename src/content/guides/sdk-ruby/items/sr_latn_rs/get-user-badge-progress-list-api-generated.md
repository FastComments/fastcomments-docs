## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badge_progress_list_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer za get_user_badge_progress_list'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# podešavanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju pomoću API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sledeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (podrazumevano nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  limit: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_user_badge_progress_list(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badge_progress_list: #{e}"
end
[inline-code-end]