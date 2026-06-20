## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer update_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfiguriraj autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uklonite komentar sa sljedećeg retka kako biste postavili prefiks za API ključ, npr. 'Bearer' (zadano je nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
updatable_comment_params = FastCommentsClient::UpdatableCommentParams.new # UpdatableCommentParams | 
opts = {
  context_user_id: 'context_user_id_example', # String | 
  do_spam_check: true, # Boolean | 
  is_live: true # Boolean | 
}

begin
  
  result = api_instance.update_comment(tenant_id, id, updatable_comment_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_comment: #{e}"
end
[inline-code-end]

---