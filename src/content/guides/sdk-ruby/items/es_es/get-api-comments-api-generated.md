## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_api_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  page: 1.2, # Flotante | 
  count: 1.2, # Flotante | 
  text_search: 'text_search_example', # Cadena | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Cadena | 
  filters: 'filters_example', # Cadena | 
  search_filters: 'search_filters_example', # Cadena | 
  sorts: 'sorts_example', # Cadena | 
  demo: true, # Booleano | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.get_api_comments(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]