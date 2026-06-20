## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| namespace | string | path | はい |  |
| component | string | path | はい |  |
| locale | string | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_translations_response.rb)

## 例

[inline-code-attrs-start title = 'get_translations の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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