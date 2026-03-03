[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO, SSO'yu uygulamak için mekanizma olarak HMAC-SHA256 şifrelemesini kullanır. Önce genel mimariyi, örnekleri ve ayrıntılı adımları inceleyeceğiz.

Benzer SSO mekanizmalarına sahip diğer sağlayıcılardan geçişle ve farklarla ilgili bazı belgeler de bulunmaktadır.

The flow looks like this:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Since Secure SSO involves full-stack development, full working code examples in Java/Spring, NodeJS/Express, and vanilla PHP are currently <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub'da</a>.

Although we use ExpressJS in the NodeJS example and Spring in the Java example there are no frameworks/libraries required in these run-times to implement FastComments SSO - the native crypto packages work.

You don't have to write any new API endpoints with FastComments SSO. Simply encrypt the user's info using your secret key and pass the payload to the comment widget.

#### API Gizli Anahtarınızı Alın

Your API Secret can be retrieved from <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">this page</a>. You can find this page also by going to My Account, clicking the API/SSO tile, and then clicking "Get API Secret Key".

#### Yorum Bileşeni Parametreleri

High-level API documentation for the comment widget can be found <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">burada</a>.

Bu parametrelerin ne anlama geldiğini daha ayrıntılı inceleyelim.

The comment widget takes a configuration object - you already pass this if you're using FastComments to pass your customer id (called tenantId).

To enable SSO, pass a new "sso" object, which must have the following parameters. The values should be generated server side.

- userDataJSONBase64: The user's data in JSON format, which is then Base64 encoded.
- verificationHash: The HMAC-SHA256 hash created from UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, in **milliseconds**. Must not be in the future, or more than two days in the past.
- loginURL: A URL that the comment widget can show to log the user in.
- logoutURL: A URL that the comment widget can show to log the user out.
- loginCallback: When provided instead of the login URL, a function that the comment widget will invoke when clicking the login button.
- logoutCallback: When provided instead of the logout URL, a function that the comment widget will invoke when clicking the logout button.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Kullanıcı Nesnesi

Kullanıcı nesnesi aşağıdaki şemayı içerir:
[inline-code-attrs-start title = 'Kullanıcı Nesnesi'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Gerekli. Maksimum 1k karakter. **/
    id: string;
    /** Gerekli. Maksimum 1k karakter. Not: Benzersiz olmalıdır. **/
    email: string;
    /** Gerekli. Maksimum 1k karakter. Not: Kullanıcı adı bir e-posta olamaz. Benzersiz olması gerekmez. **/
    username: string;
    /** Opsiyonel. URL'ler için maksimum 3k karakter. Varsayılan, e-postaya dayalı gravatar'dandır. 64 ile kodlanmış resimleri destekler; bu durumda limit 50k karakterdir. **/ 
    avatar?: string;
    /** Opsiyonel. Varsayılan false. **/
    optedInNotifications?: boolean;
    /** Opsiyonel. Varsayılan false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opsiyonel. Maksimum 100 karakter. Bu etiket isimlerinin yanında gösterilecektir. Uygulanabilir olduğunda varsayılan Yönetici/Moderatör'dür. **/
    displayLabel?: string;
    /** Opsiyonel. Maksimum 500 karakter. Bu, kullanıcı adı yerine gösterilecektir. **/
    displayName?: string;
    /** Opsiyonel. Maksimum 2k karakter. Kullanıcının adı buraya bağlanacaktır. **/
    websiteUrl?: string;
    /** Opsiyonel. Kullanıcı başına 100 gruba kadar. Bir grup kimliği 50 karakterden uzun olamaz. **/
    groupIds?: string[];
    /** Opsiyonel. Kullanıcıyı yönetici olarak belirtir. **/
    isAdmin?: boolean;
    /** Opsiyonel. Kullanıcıyı moderatör olarak belirtir. **/
    isModerator?: boolean;
    /** Opsiyonel, varsayılan true. Kullanıcının profilindeki "activity" sekmesini etkinleştirmek için false olarak ayarlayın. **/
    isProfileActivityPrivate?: boolean;
    /** Opsiyonel, varsayılan false. Profil yorumlarını devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileCommentsPrivate?: boolean;
    /** Opsiyonel, varsayılan false. Bu kullanıcıya doğrudan mesaj göndermeyi devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileDMDisabled?: boolean;
    /** Kullanıcı rozetleri için opsiyonel yapılandırma. **/
    badgeConfig?: {
        /** Atanacak genel rozet kimlikleri dizisi. 30 rozet ile sınırlıdır. Sıra korunur. **/
        badgeIds: string[];
        /** Geçerli sayfaya (urlId) özgü rozet kimlikleri dizisi. Yalnızca atandığı sayfada gösterilir. **/
        pageBadgeIds?: string[];
        /** True ise gösterilen mevcut rozetlerin yerini alır. Genel ve sayfa-açısına özgü rozetler bağımsız olarak üzerine yazılır. **/
        override?: boolean;
        /** True ise, rozet görüntüleme özelliklerini tenant yapılandırmasından günceller. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatörler ve Yöneticiler

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Bildirimler

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Ayrıca, kullanıcıların yalnızca uygulama içi bildirimler yerine abone oldukları sayfalardaki etkinlikler için bildirim e-postası almalarını istiyorsanız, `optedInSubscriptionNotifications` değerini `true` olarak ayarlayın.

#### VIP Kullanıcılar ve Özel Etiketler

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Kimlik Doğrulanmamış kullanıcılar

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Kullanıcı Verilerinin Seri Haline Getirilmesi ve Hash'lenmesi için Doğrudan Örnekler

Daha fazla ayrıntı örnekleri js için <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">burada</a>, java için <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">burada</a> ve php için <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">burada</a>.

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">destek sayfasını</a>.