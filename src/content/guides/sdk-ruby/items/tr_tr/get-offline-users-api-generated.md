Sayfadaki VE şu anda çevrimiçi olmayan önceki yorumcular. displayName'e göre sıralanır.
Bir "Members" bölümü oluşturmak için /users/online tükendiğinde bunu kullanın.
commenterName üzerinde imleç paginasyonu: sunucu kısmi {tenantId, urlId, commenterName}
indeksini afterName'den ileri doğru $gt ile yürütür, $skip maliyeti yok.

## Parameters

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini gönderin. |
| afterUserId | string | query | Hayır | Beraberlik durumunu çözen imleç: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, aynı ada sahip kayıtların atlanmaması için gereklidir. |

## Response

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Example

[inline-code-attrs-start title = 'get_offline_users Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
opts = {
  after_name: 'after_name_example', # String | İmleç: önceki yanıttan nextAfterName değerini gönderin.
  after_user_id: 'after_user_id_example' # String | Beraberlik durumunu çözen imleç: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, aynı ada sahip kayıtların atlanmaması için gereklidir.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]