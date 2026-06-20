---
Bir sayfanın şu anda çevrimiçi izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler, saymadığımız anonim izleyiciler dahil).

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini gönderin. |
| afterUserId | string | query | Hayır | Berabere durumunu çözmek için imleç: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim eşleşmeleri nedeniyle kayıtların düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_online_users Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
opts = {
  after_name: 'after_name_example', # String | İmleç: önceki yanıttan nextAfterName değerini gönderin.
  after_user_id: 'after_user_id_example' # String | Berabere durumunu çözmek için imleç: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim eşleşmeleri nedeniyle kayıtların düşmemesi için gereklidir.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---