[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Güvenli SSO, SSO'yu uygulamak için mekanizma olarak HMAC-SHA256 şifrelemesini kullanır. Önce genel mimariyi ele alacağız, örnekler sağlayacağız ve ayrıntılı adımları açıklayacağız.

Ayrıca benzer SSO mekanizmalarına sahip diğer sağlayıcılardan geçişle ilgili bazı belgeler ve farklar bulunmaktadır.

Akış şu şekilde görünür:

<div class="screenshot white-bg">
    <div class="title">Güvenli SSO Akışı</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Güvenli SSO Diyagramı" />
</div>

Güvenli SSO tam yığın geliştirme (full-stack) içerdiği için, Java/Spring, NodeJS/Express ve sade PHP'de tam çalışan kod örnekleri şu anda <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub'da</a> bulunmaktadır.

NodeJS örneğinde ExpressJS'i ve Java örneğinde Spring'i kullanmamıza rağmen, FastComments SSO'yu uygulamak için bu çalışma zamanlarında herhangi bir framework/kütüphane zorunlu değildir - yerel crypto paketleri yeterlidir.

FastComments SSO ile herhangi bir yeni API uç noktası yazmanız gerekmez. Kullanıcının bilgilerini gizli anahtarınızı kullanarak şifreleyin ve yükü yorum widget'ına iletin.

#### API Gizli Anahtarınızı Alın

API Gizli Anahtarınız <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">bu sayfadan</a> alınabilir. Bu sayfayı ayrıca My Account'a giderek, API/SSO kutucuğuna tıklayarak ve ardından "Get API Secret Key"e tıklayarak da bulabilirsiniz.

#### Yorum Widget Parametreleri

Yorum widget'ı için üst düzey API dokümantasyonu <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">burada</a> bulunabilir.

Bu parametrelerin ne anlama geldiğini daha ayrıntılı inceleyelim.

Yorum widget'ı bir yapılandırma nesnesi alır - FastComments kullanıyorsanız zaten müşteri kimliğinizi (tenantId olarak adlandırılır) geçirmek için bunu sağlıyorsunuz.

SSO'yu etkinleştirmek için, sunucu tarafında oluşturulması gereken aşağıdaki parametrelere sahip yeni bir "sso" nesnesi geçin.

- userDataJSONBase64: Kullanıcının JSON formatındaki verileri, daha sonra Base64 ile kodlanmış halde.
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64'den oluşturulan HMAC-SHA256 hash.
- timestamp: Epoch zaman damgası, **milisaniye** cinsinden. Gelecekte olmamalı veya iki günden fazla geçmiş olmamalı.
- loginURL: Yorum widget'ının kullanıcıyı oturum açması için gösterebileceği bir URL.
- logoutURL: Yorum widget'ının kullanıcıyı oturum kapatması için gösterebileceği bir URL.
- loginCallback: Giriş URL'si yerine sağlandığında, yorum widget'ının giriş düğmesine tıklandığında çağıracağı bir fonksiyon.
- logoutCallback: Çıkış URL'si yerine sağlandığında, yorum widget'ının çıkış düğmesine tıklandığında çağıracağı bir fonksiyon.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Kullanıcı Nesnesi'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Zorunlu. Maksimum 1k karakter. **/
    id: string;
    /** Zorunlu. Maksimum 1k karakter. Not: Benzersiz olmalıdır. **/
    email: string;
    /** Zorunlu. Maksimum 1k karakter. Not: Kullanıcı adı bir e-posta olamaz. Benzersiz olması gerekmez. **/
    username: string;
    /** İsteğe bağlı. URL'ler için maksimum 3k karakter. Varsayılan, e-posta bazlı gravatardan alınır. 64 kodlu (Base64) resimleri destekler; bu durumda sınır 50k karakterdir. **/ 
    avatar?: string;
    /** İsteğe bağlı. Varsayılan false. **/
    optedInNotifications?: boolean;
    /** İsteğe bağlı. Varsayılan false. **/
    optedInSubscriptionNotifications?: boolean;
    /** İsteğe bağlı. Maksimum 100 karakter. Bu etiket isimlerinin yanında gösterilecektir. Uygun olduğunda varsayılan Administrator/Moderator'dür. **/
    displayLabel?: string;
    /** İsteğe bağlı. Maksimum 500 karakter. Bu, kullanıcı adı yerine gösterilecektir. **/
    displayName?: string;
    /** İsteğe bağlı. Maksimum 2k karakter. Kullanıcının adı buraya bağlantı verilir. **/
    websiteUrl?: string;
    /** İsteğe bağlı. Kullanıcı başına en fazla 100 grup. Bir grup id'si 50 karakterden uzun olamaz. **/
    groupIds?: string[];
    /** İsteğe bağlı. Kullanıcıyı yönetici olarak belirtir. **/
    isAdmin?: boolean;
    /** İsteğe bağlı. Kullanıcıyı moderatör olarak belirtir. **/
    isModerator?: boolean;
    /** İsteğe bağlı, varsayılan true. Kullanıcının profilindeki "etkinlik" sekmesini etkinleştirmek için false olarak ayarlayın. **/
    isProfileActivityPrivate?: boolean;
    /** İsteğe bağlı, varsayılan false. Profil yorumlarını devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileCommentsPrivate?: boolean;
    /** İsteğe bağlı, varsayılan false. Bu kullanıcıya doğrudan mesaj göndermeyi devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notifications

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### VIP Users & Special Labels

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Unauthenticated users

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Direct Examples for Serializing and Hashing User Data

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.