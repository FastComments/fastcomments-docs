## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_moderator_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer create_moderator'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uklonite komentar sa sledeće linije da biste postavili prefiks za API ključ, npr. 'Bearer' (podrazumevano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_moderator_body = FastCommentsClient::CreateModeratorBody.new({name: 'name_example', email: 'email_example'}) # CreateModeratorBody | 

begin
  
  result = api_instance.create_moderator(tenant_id, create_moderator_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_moderator: #{e}"
end
[inline-code-end]