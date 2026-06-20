Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat`'in true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkili. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı büyük/küçük harf duyarsız başlık önek filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce) veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | true ise, en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_pages_public Örnek'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir.
  limit: 56, # Integer | 1..200, varsayılan 50
  q: 'q_example', # String | İsteğe bağlı büyük/küçük harf duyarsız başlık önek filtresi.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik).
  has_comments: true # Boolean | true ise, en az bir yorumu olan sayfaları döndürür.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]