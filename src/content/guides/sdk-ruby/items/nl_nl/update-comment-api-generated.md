## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nee |  |
| doSpamCheck | boolean | query | Nee |  |
| isLive | boolean | query | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'update_comment Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutel autorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaarmarkering van de volgende regel weg om een prefix voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard nil)
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