## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nee |  |

## Response

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'update_notification Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# autorisatie instellen
FastCommentsClient.configure do |config|
  # Configureer API-sleutelautorisatie: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Haal de commentaartekens van de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. 'Bearer' (standaard nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_notification_body = FastCommentsClient::UpdateNotificationBody.new # UpdateNotificationBody | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.update_notification(tenant_id, id, update_notification_body, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_notification: #{e}"
end
[inline-code-end]