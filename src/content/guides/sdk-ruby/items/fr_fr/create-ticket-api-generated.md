## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Oui |  |

## Réponse

Renvoie : [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_ticket200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_ticket'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'authentification
FastCommentsClient.configure do |config|
  # Configurer l'authentification par clé API : api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par exemple 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
create_ticket_body = FastCommentsClient::CreateTicketBody.new({subject: 'subject_example'}) # CreateTicketBody | 

begin
  
  result = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_ticket: #{e}"
end
[inline-code-end]