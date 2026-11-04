Page Reacts, kullanıcılarınızın bir sayfayı beğenmesini veya kendi tepki görselleri setinizle tepki vermesini sağlar. [Page Reacts widget'ı](/guide-page-reacts.html) ve Floating Likes widget'ı bu endpoint'ler üzerine kuruludur ve kendi beğenme butonunuzu oluşturmak için bunları kendiniz çağırabilirsiniz.

Bu kılavuzun geri kalanının aksine, Page Reacts endpoint'leri herkese açıktır. Bunlar kullanıcılarınızın tarayıcılarından çağrılır, API anahtarı gerektirmez ve API kredisi harcamaz. Her tepki, isteği yapan kullanıcıya aittir, dolayısıyla bir kullanıcı yalnızca kendi tepkisini ekleyebilir veya kaldırabilir.

İki endpoint seti vardır:

- `/page-reacts/v1/likes/:tenantId` - kullanıcı başına sayfa başına tek bir "beğeni". Bunları bir beğenme butonu için kullanın.
- `/page-reacts/v2/:tenantId` - sayfa başına birden fazla tepki, her biri sizin seçtiğiniz kısa bir `id` ile tanımlanır (örneğin `heart` veya `laugh`).

Her ikisi de SDK'larımızda `PublicApi`'nin bir parçası olarak mevcuttur; örneğin [JavaScript SDK](/guide-sdk-javascript.html)'sında `getV1PageLikes`, `createV1PageReact` ve `deleteV1PageReact`.

### Kimin Tepki Verdiğini Gösterme

Beğenen veya tepki veren kullanıcıları, kullanıcı adları ve avatarlarıyla birlikte en yeniden eskiye doğru almak için `GET` endpoint'lerine `includeUserInfo=public` parametresini iletin. Profil etkinliğini gizli yapmış kullanıcılar ve anonim kullanıcılar bu listede yer almaz. En fazla 100 tepki döndürülür.

SSO kullanıcıları, SSO payload'ınız `isProfileActivityPrivate` değerini `false` olarak ayarlamadıkça gizli profil etkinliğiyle oluşturulur; bu nedenle bu listelerde görünmesini istediğiniz kullanıcılar için bunu ayarlayın.

SSO gerektiren sayfalarda, yalnızca sayfayı görüntüleyebilen SSO kullanıcıları kimin tepki verdiğini listeleyebilir. Diğerleri `sso-required` veya `page-forbidden` koduyla bir `403` hatası alır.

Sunucunuzdan, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) ve [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) endpoint'leri bir API anahtarıyla aynı listeleri döndürür ve ayrıca gizli profilli kullanıcıları dahil etmek için `includeUserInfo=include-private` parametresini kabul eder.

### Kullanıcıyı Tanımlama

Tepkiler, isteği yapan kullanıcıya bağlıdır:

- **SSO kullanıcıları:** yorum widget'ına verdiğiniz aynı SSO nesnesinin URI kodlanmış JSON'u olarak ayarlanmış `sso` sorgu parametresini iletin. Bkz. [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonim kullanıcılar:** `sso` parametresi ve FastComments oturumu olmadığında, sunucu tarayıcıya FastComments oturum çerezinde saklanan anonim bir id atar. Çerezin istekler arasında korunması için istekleri `credentials: 'include'` ile gönderin. Üçüncü taraf çerezlerini engelleyen tarayıcılar anonim id'yi korumaz, bu nedenle her kullanıcının güvenilir bir şekilde tanınması gerekiyorsa SSO kullanın.

### urlId

`urlId`, yorumlarda olduğu gibi sayfayı tanımlar. Beğenilerin ve yorumların aynı sayfada sayılması için yorum widget'ına verdiğiniz aynı `urlId`'yi kullanın. URI kodlamasını yapmayı unutmayın.

[inline-code-attrs-start title = 'Beğenme Butonu Örneği'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// İsteğe bağlı, SSO kullanıcıları için. Yorum widget'ının "sso" seçeneğine verdiğiniz aynı nesne.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]