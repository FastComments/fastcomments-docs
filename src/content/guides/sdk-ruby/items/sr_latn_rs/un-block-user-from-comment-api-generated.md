## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/unblock_success.rb)

## Primer

[inline-code-attrs-start title = 'un_block_user_from_comment Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# podešavanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sledeću liniju da biste postavili prefiks za API ključ, npr. 'Bearer' (podrazumevano nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
un_block_from_comment_params = FastCommentsClient::UnBlockFromCommentParams.new # UnBlockFromCommentParams | 
opts = {
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example' # String | 
}

begin
  
  result = api_instance.un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->un_block_user_from_comment: #{e}"
end
[inline-code-end]