## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/patch_page_a_p_i_response.rb)

## Eksempel

[inline-code-attrs-start title = 'patch_page Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt godkendelse
FastCommentsClient.configure do |config|
  # Konfigurer API-nøglegodkendelse: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentartegnet fra følgende linje for at angive et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_page_data = FastCommentsClient::UpdateAPIPageData.new # UpdateAPIPageData | 

begin
  
  result = api_instance.patch_page(tenant_id, id, update_api_page_data)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_page: #{e}"
end
[inline-code-end]

---