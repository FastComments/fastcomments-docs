## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| namespace | string | path | Evet |  |
| component | string | path | Evet |  |
| locale | string | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_translations_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_translations Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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

---