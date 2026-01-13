## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| updateComments | string | query | Nej |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'replace_tenant_user Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisation: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentartegnet fra følgende linje for at angive et præfiks for API-nøglen, f.eks. 'Bearer' (standard: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
replace_tenant_user_body = FastCommentsClient::ReplaceTenantUserBody.new({username: 'username_example', email: 'email_example'}) # ReplaceTenantUserBody | 
opts = {
  update_comments: 'update_comments_example' # String | 
}

begin
  
  result = api_instance.replace_tenant_user(tenant_id, id, replace_tenant_user_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->replace_tenant_user: #{e}"
end
[inline-code-end]

---