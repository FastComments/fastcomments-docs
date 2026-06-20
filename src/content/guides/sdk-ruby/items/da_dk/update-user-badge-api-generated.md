## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_success_response.rb)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på update_user_badge'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt godkendelse
FastCommentsClient.configure do |config|
  # Konfigurer API-nøglegodkendelse: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentartegnet på følgende linje for at sætte et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_user_badge_params = FastCommentsClient::UpdateUserBadgeParams.new # UpdateUserBadgeParams | 

begin
  
  result = api_instance.update_user_badge(tenant_id, id, update_user_badge_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_user_badge: #{e}"
end
[inline-code-end]