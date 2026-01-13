Burada erişim kontrolünü ayarlamak için FastComments API'sini nasıl çağıracağımızı adım adım göstereceğiz.

Başlamadan önce, `Group` yapısını açıkça oluşturmamız gerekmediğini unutmayın. Gruplar sadece `Users` ve `Pages`'e eklenen tanımlayıcılardır. Bir kullanıcıya veya sayfaya grup eklemek, grubu otomatik olarak "oluşturur".

First, let's create two users, `User A` and `User B`, we'll start them out in `Group X`:

[inline-code-attrs-start title = 'Kullanıcı A Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Kullanıcı B Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Now let's create a `Page`. We'll call it our `Confidential Page`, and so far none of these users will have access to it as it will
be in the group `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Sayfa POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Kullanıcı A ve B şu anda yeni sayfaya **ERİŞEMİYORLAR**. Ancak aynı grup `GROUP-X` içinde oldukları için birbirlerini `@mention` edebilirler.

Let's update `User B` so they can now access the page:

[inline-code-attrs-start title = 'Kullanıcı B Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` artık her iki gruba ait. Kullanıcılarımız hâlâ birbirlerini `@mention` edebilir, ancak yalnızca `User B` gizli sayfamızı görüntüleyebilir.

Let's make it so `User B` can only view the confidential page:

[inline-code-attrs-start title = 'Kullanıcı B Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Şimdi gizli sayfayı görüntüleyebiliyorlar, ancak kullanıcılarımızın hiçbiri birbirini `@mention` edemez; çünkü farklı gruplardalar.

However, any user that is not part of access control **will be able to access our page**. To prevent this, ensure no SSO Users have
their `groupIds` set to null. For example, let's create `User C`, which has access to everything:

[inline-code-attrs-start title = 'Kullanıcı C Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`groupIds`'i null olarak ayarlayarak, erişim kontrolü tarafından kısıtlanmadıklarını belirtiriz.

Now, let's create a page that everyone has access to:

[inline-code-attrs-start title = 'Sayfa POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`accessibleByGroupIds`'i null olarak ayarlayarak, bu `Page`'in erişim kontrolü ile yönetilmediğini ve her iki kullanıcının da ona erişebildiğini belirtiriz.

This completes our API walk-through for Access Control.

---