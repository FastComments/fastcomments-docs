## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | string | query | Ne |  |

## Odgovor

Vrne: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Primer

[inline-code-attrs-start title = 'replace_tenant_user Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# nastavitev avtorizacije
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentirajte naslednjo vrstico, če želite nastaviti predpono za API ključ, npr. 'Bearer' (privzeto nil)
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