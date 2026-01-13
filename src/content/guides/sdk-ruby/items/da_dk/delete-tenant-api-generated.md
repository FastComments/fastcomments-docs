## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| sure | string | query | Nej |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'delete_tenant Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsætning af autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgle autorisation: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren fra følgende linje for at sætte et præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  sure: 'sure_example' # String | 
}

begin
  
  result = api_instance.delete_tenant(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_tenant: #{e}"
end
[inline-code-end]