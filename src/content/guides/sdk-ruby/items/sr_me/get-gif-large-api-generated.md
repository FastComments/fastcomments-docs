## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| largeInternalURLSanitized | string | query | Да |  |

## Одговор

Враћа: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/gif_get_large_response.rb)

## Пример

[inline-code-attrs-start title = 'get_gif_large Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
large_internal_url_sanitized = 'large_internal_url_sanitized_example' # String | 

begin
  
  result = api_instance.get_gif_large(tenant_id, large_internal_url_sanitized)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gif_large: #{e}"
end
[inline-code-end]