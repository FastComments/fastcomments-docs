## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_site_search_response.rb)

## Пример

[inline-code-attrs-start title = 'get_search_sites Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_search_sites(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_sites: #{e}"
end
[inline-code-end]