Page Reacts, kullanıcılarınızın bir sayfayı beğenmesini veya kendi reaksiyon görselleri setinizle ona tepki vermesini sağlar. [Page Reacts widget](/guide-page-reacts.html) ve Floating Likes widget'ı bu uç noktalara dayanır ve kendi beğen düğmenizi oluşturmak için bunları kendiniz de çağırabilirsiniz.

Diğer kılavuz bölümlerinin aksine, Page Reacts uç noktaları herkese açıktır. Kullanıcıların tarayıcılarından çağrılır, API anahtarı gerektirmez ve API kredisi harcamaz. Her reaksiyon, isteği yapan kullanıcıya aittir, bu yüzden bir kullanıcı yalnızca kendi beğenilerini ekleyebilir veya kaldırabilir.

İki set uç nokta vardır:

- `/page-reacts/v1/likes/:tenantId` - bir sayfa başına kullanıcı başına tek bir "beğeni". Beğen düğmesi için bunları kullanın.
- `/page-reacts/v2/:tenantId` - bir sayfa başına birden fazla reaksiyon, her biri sizin seçtiğiniz kısa bir `id` ile tanımlanır (örneğin `heart` veya `laugh`).

Her ikisi de `PublicApi` kapsamında SDK'larımızda mevcuttur; örneğin [JavaScript SDK](/guide-sdk-javascript.html) içinde `getV1PageLikes`, `createV1PageReact` ve `deleteV1PageReact`.

### Kullanıcıyı Tanımlama

Reaksiyonlar, isteği yapan kullanıcıya bağlanır:

- **SSO kullanıcıları:** yorum widget'ına verdiğiniz aynı SSO nesnesinin URI kodlu JSON'u olan `sso` sorgu parametresini gönderin. Bkz. [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonim kullanıcılar:** `sso` parametresi ve FastComments oturumu yoksa, sunucu tarayıcıya FastComments oturum çerezi içinde saklanan anonim bir kimlik atar. Çerezlerin istekler arasında korunması için `credentials: 'include'` ile istek gönderin. Üçüncü taraf çerezlerini engelleyen tarayıcılar anonim kimliği tutmayacaktır; bu yüzden her kullanıcının güvenilir bir şekilde tanınması gerektiğinde SSO kullanın.

### urlId

`urlId`, sayfayı tanımlar; yorumlar için olduğu gibi aynı şekilde çalışır. Beğeniler ve yorumların aynı sayfada sayılması için yorum widget'ına verdiğiniz aynı `urlId`'yi kullanın. URI kodlamayı unutmayın.

[inline-code-attrs-start title = 'Beğen Düğmesi Örneği'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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