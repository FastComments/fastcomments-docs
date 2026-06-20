## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Yes |  |
| component | string | path | Yes |  |
| locale | string | query | No |  |
| useFullTranslationIds | boolean | query | No |  |

## Response

Returns: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_translations_response.rb)

## Example

[inline-code-attrs-start title = 'get_translations Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
namespace = 'namespace_example' # String | 
component = 'component_example' # String | 
opts = {
  locale: 'locale_example', # String | 
  use_full_translation_ids: true # Boolean | 
}

begin
  
  result = api_instance.get_translations(namespace, component, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_translations: #{e}"
end
[inline-code-end]
