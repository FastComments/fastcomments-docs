## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Risposta

Restituisce: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_comments_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# impostazione dell'autorizzazione
FastCommentsClient.configure do |config|
  # Configura l'autorizzazione con API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Decommenta la riga seguente per impostare un prefisso per la API key, es. 'Bearer' (predefinito nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 56, # Integer | 
  limit: 56, # Integer | 
  skip: 56, # Integer | 
  as_tree: true, # Boolean | 
  skip_children: 56, # Integer | 
  limit_children: 56, # Integer | 
  max_tree_depth: 56, # Integer | 
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example', # String | 
  context_user_id: 'context_user_id_example', # String | 
  hash_tag: 'hash_tag_example', # String | 
  parent_id: 'parent_id_example', # String | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  from_date: 789, # Integer | 
  to_date: 789 # Integer | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]