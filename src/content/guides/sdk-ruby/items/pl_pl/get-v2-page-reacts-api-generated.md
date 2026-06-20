## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_reacts.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_v2_page_reacts'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 

begin
  
  result = api_instance.get_v2_page_reacts(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_reacts: #{e}"
end
[inline-code-end]