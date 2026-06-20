## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | כן |  |
| component | string | path | כן |  |
| locale | string | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |

## תגובה

מחזיר: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_translations_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_translations'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
namespace = 'namespace_example' # מחרוזת | 
component = 'component_example' # מחרוזת | 
opts = {
  locale: 'locale_example', # מחרוזת | 
  use_full_translation_ids: true # בוליאני | 
}

begin
  
  result = api_instance.get_translations(namespace, component, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_translations: #{e}"
end
[inline-code-end]

---