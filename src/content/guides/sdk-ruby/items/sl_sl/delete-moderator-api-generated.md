## Parametri

| Ime | Vrsta | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| sendEmail | string | query | Ne |  |

## Odziv

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer delete_moderator'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# nastavitev avtorizacije
FastCommentsClient.configure do |config|
  # Konfigurirajte avtorizacijo z API ključem: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentirajte naslednjo vrstico za nastavitev prefiksa za API ključ, npr. 'Bearer' (privzeto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  send_email: 'send_email_example' # String | 
}

begin
  
  result = api_instance.delete_moderator(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_moderator: #{e}"
end
[inline-code-end]

---