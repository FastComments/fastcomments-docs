Burada FastComments API'sini erişim kontrolü kurmak için çağırma sürecini adım adım ele alacağız.

Başlamadan önce, açıkça bir `Group` yapısı oluşturmamıza gerek olmadığını belirtelim. Gruplar, `Users` ve `Pages` öğelerine eklenen basit tanımlayıcılardır. Bir kullanıcıya veya sayfaya grup eklemek, grubu otomatik olarak "oluşturur".

Öncelikle, `User A` ve `User B` olmak üzere iki kullanıcı oluşturalım; ikisini de `Group X` içinde başlatacağız:

[inline-code-attrs-start title = 'User A Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'User B Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Şimdi bir `Page` oluşturalım. Buna `Confidential Page` (Gizli Sayfa) diyelim; şu ana kadar bu kullanıcıların hiçbiri ona erişemeyecek çünkü sayfa `CONFIDENTIAL` grubunda olacak:

[inline-code-attrs-start title = 'Page POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

A ve B kullanıcıları şu anda yeni sayfaya **ERİŞEMEZ**. Ancak, aynı grupta oldukları için, `GROUP-X`, birbirlerini `@mention` (bahsedebilirler).

`User B` kullanıcısını, artık sayfaya erişebilecek şekilde güncelleyelim:

[inline-code-attrs-start title = 'User B Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` artık her iki gruba da ait. Kullanıcılarımız hâlâ birbirlerini `@mention` yapabilir, ancak yalnızca `User B` gizli sayfamızı görüntüleyebilir.

`User B`'nin yalnızca gizli sayfayı görüntüleyebilmesini sağlayalım:

[inline-code-attrs-start title = 'User B Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Artık gizli sayfayı görüntüleyebilir, ancak kullanıcılarımızın hiçbiri birbirini `@mention` yapamaz, çünkü farklı gruplardadırlar.

Ancak, erişim kontrolünün bir parçası olmayan herhangi bir kullanıcı **sayfamıza erişebilecektir**. Bunu önlemek için, hiçbir SSO kullanıcısının `groupIds` değerinin null olarak ayarlanmadığından emin olun. Örneğin, her şeye erişimi olan `User C` kullanıcısını oluşturalım:

[inline-code-attrs-start title = 'User C Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

`groupIds` değerini null olarak ayarlayarak, erişim kontrolüyle sınırlı olmadıklarını belirtmiş oluruz.

Şimdi, herkesin erişebildiği bir sayfa oluşturalım:

[inline-code-attrs-start title = 'Page POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

`accessibleByGroupIds` değerini null olarak ayarlayarak, bu `Page`'in erişim kontrolü aracılığıyla yönetilmediğini ve her iki kullanıcının da ona erişebildiğini belirtmiş oluruz.

Bu, Erişim Kontrolü için API turumuzu tamamlıyor.