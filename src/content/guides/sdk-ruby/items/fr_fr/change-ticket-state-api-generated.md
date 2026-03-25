## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Renvoie : [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_ticket_state200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de change_ticket_state'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'authentification
FastCommentsClient.configure do |config|
  # Configurer l'authentification par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par exemple 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
id = 'id_example' # String | 
change_ticket_state_body = FastCommentsClient::ChangeTicketStateBody.new({state: 37}) # ChangeTicketStateBody | 

begin
  
  result = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->change_ticket_state: #{e}"
end
[inline-code-end]