[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Güvenli SSO, SSO'yu uygulamak için mekanizma olarak HMAC-SHA256 şifrelemesini kullanır. Önce genel mimariyi ele alacağız, ardından örnekler ve ayrıntılı adımlar sunacağız.

Benzer SSO mekanizmalarına sahip diğer sağlayıcılardan geçiş yapmaya ve aralarındaki farklara dair bazı belgeler de mevcuttur.

Akış şu şekildedir:

<div class="screenshot white-bg">
    <div class="title">Güvenli SSO Akışı</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Güvenli SSO Diyagramı" />
</div>

Güvenli SSO tam yığın (full-stack) geliştirme gerektirdiğinden, Java/Spring, NodeJS/Express ve saf PHP için eksiksiz çalışan kod örnekleri şu anda <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub'da</a> bulunmaktadır.

NodeJS örneğinde ExpressJS ve Java örneğinde Spring kullanmamıza rağmen, FastComments SSO'yu uygulamak için bu çalışma ortamlarında herhangi bir framework/kütüphane gerekmez - yerel crypto paketleri yeterlidir.

FastComments SSO ile yeni API endpoint'leri yazmanız gerekmez. Sadece kullanıcının bilgilerini gizli anahtarınızla şifreleyin ve payload'ı yorum widget'ına iletin.

#### API Gizli Anahtarınızı Alın

API Gizli Anahtarınızı <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">bu sayfadan</a> alabilirsiniz. Bu sayfaya Hesabım bölümüne gidip API/SSO kutucuğuna ve ardından "Get API Secret Key" seçeneğine tıklayarak da ulaşabilirsiniz.

#### Yorum Widget'ı Parametreleri

Yorum widget'ının üst düzey API belgelerine <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">buradan</a> ulaşabilirsiniz.

Bu parametrelerin ne anlama geldiğine daha yakından bakalım.

Yorum widget'ı bir yapılandırma nesnesi alır - FastComments kullanırken müşteri kimliğinizi (tenantId olarak adlandırılır) iletmek için bunu zaten iletiyorsunuzdur.

SSO'yu etkinleştirmek için, aşağıdaki parametreleri içermesi gereken yeni bir "sso" nesnesi iletin. Değerler sunucu tarafında oluşturulmalıdır.

- userDataJSONBase64: JSON formatındaki kullanıcı verisi, ardından Base64 ile kodlanır.
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64 üzerinden oluşturulan HMAC-SHA256 hash'i.
- timestamp: Epoch zaman damgası, **milisaniye** cinsinden. Gelecekte veya iki günden daha eski bir tarihte olmamalıdır.
- loginURL: Yorum widget'ının kullanıcıyı oturum açmaya yönlendirmek için gösterebileceği bir URL.
- logoutURL: Yorum widget'ının kullanıcıyı oturumu kapatmaya yönlendirmek için gösterebileceği bir URL.
- loginCallback: Oturum açma URL'si yerine sağlandığında, oturum açma düğmesine tıklandığında yorum widget'ının çağıracağı bir fonksiyon.
- logoutCallback: Oturum kapatma URL'si yerine sağlandığında, oturum kapatma düğmesine tıklandığında yorum widget'ının çağıracağı bir fonksiyon.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Güvenli SSO İstemci Kodu'; isFunctional = false; code-example-end]

#### Kullanıcı Nesnesi

Kullanıcı nesnesi şu şemayı içerir:
[inline-code-attrs-start title = 'Kullanıcı Nesnesi'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Zorunlu. En fazla 1k karakter. **/
    id: string;
    /** Zorunlu. En fazla 1k karakter. Not: Benzersiz olmalıdır. **/
    email: string;
    /** Zorunlu. En fazla 1k karakter. Not: Kullanıcı adı bir e-posta olamaz. Benzersiz olmak zorunda değildir. **/
    username: string;
    /** İsteğe bağlı. URL'ler için en fazla 3k karakter. Varsayılan, e-postaya göre gravatar'dan alınır. 64 kodlu görselleri destekler; bu durumda sınır 50k karakterdir. **/ 
    avatar?: string;
    /** İsteğe bağlı. Varsayılan false. **/
    optedInNotifications?: boolean;
    /** İsteğe bağlı. Varsayılan false. **/
    optedInSubscriptionNotifications?: boolean;
    /** İsteğe bağlı. En fazla 100 karakter. Bu etiket, kullanıcının adının yanında gösterilecektir. Uygun olduğunda varsayılan Administrator/Moderator'dur. **/
    displayLabel?: string;
    /** İsteğe bağlı. En fazla 500 karakter. Bu, kullanıcı adı yerine gösterilecektir. **/
    displayName?: string;
    /** İsteğe bağlı. En fazla 2k karakter. Kullanıcının adı buna bağlanacaktır. **/
    websiteUrl?: string;
    /** İsteğe bağlı. Kullanıcı başına en fazla 100 grup. Bir grup kimliği 50 karakterden uzun olamaz. **/
    groupIds?: string[];
    /** İsteğe bağlı. Kullanıcıyı yönetici olarak işaretler. **/
    isAdmin?: boolean;
    /** İsteğe bağlı. Kullanıcıyı moderatör olarak işaretler. **/
    isModerator?: boolean;
    /** İsteğe bağlı, varsayılan true. Kullanıcının profilindeki "activity" sekmesini etkinleştirmek için false olarak ayarlayın. **/
    isProfileActivityPrivate?: boolean;
    /** İsteğe bağlı, varsayılan false. Profil yorumlarını devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileCommentsPrivate?: boolean;
    /** İsteğe bağlı, varsayılan false. Bu kullanıcıya doğrudan mesaj göndermeyi devre dışı bırakmak için true olarak ayarlayın. **/
    isProfileDMDisabled?: boolean;
    /** Kullanıcı rozetleri için isteğe bağlı yapılandırma. **/
    badgeConfig?: {
        /** Atanacak global rozet kimlikleri dizisi. 30 rozetle sınırlıdır. Sıra korunur. **/
        badgeIds: string[];
        /** Geçerli sayfaya (urlId) özel rozet kimlikleri dizisi. Yalnızca atanan sayfada görüntülenir. **/
        pageBadgeIds?: string[];
        /** true ise, mevcut görüntülenen rozetlerin yerini alır. Global ve sayfa kapsamlı olanlar bağımsız olarak geçersiz kılınır. **/
        override?: boolean;
        /** true ise, rozet görüntüleme özelliklerini kiracı yapılandırmasından günceller. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatörler ve Yöneticiler

Yöneticiler ve moderatörler için, `SSOUser` nesnesinde sırasıyla `isAdmin` veya `isModerator` bayraklarını iletin.

#### Bildirimler

Bildirimleri etkinleştirmek veya devre dışı bırakmak için `optedInNotifications` değerini sırasıyla `true` veya `false` olarak ayarlayın. Kullanıcı bu değeri SSO payload'ında içeren sayfayı ilk kez yüklediğinde, bildirim ayarları güncellenecektir.

Ayrıca, kullanıcıların abone oldukları sayfalardaki etkinlikler için bildirim e-postaları almasını istiyorsanız (yalnızca uygulama içi bildirimler yerine), `optedInSubscriptionNotifications` değerini `true` olarak ayarlayın.

#### VIP Kullanıcılar ve Özel Etiketler

İsteğe bağlı "displayLabel" alanını kullanarak kullanıcının adının yanında özel bir etiket görüntüleyebilirsiniz.

#### Kimliği Doğrulanmamış Kullanıcılar

Kimliği doğrulanmamış bir kullanıcıyı temsil etmek için, userDataJSONBase64, verificationHash veya timestamp alanlarını doldurmayın. Bir loginURL sağlayın.

Bu kullanıcılar yorum yapamaz; bunun yerine bir oturum açma mesajı (yapılandırmaya bağlı olarak mesaj, bağlantı veya düğme) sunulur.

#### Kullanıcı Verilerini Serileştirme ve Hashleme İçin Doğrudan Örnekler

Örnek olarak daha fazla ayrıntı <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">burada</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">burada</a> (java) ve <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">burada</a> (php) bulunmaktadır.

Herhangi bir entegrasyonun karmaşık ve zahmetli bir süreç olabileceğini biliyoruz. Temsilcinize ulaşmaktan veya <a href="https://fastcomments.com/auth/my-account/help" target="_blank">destek sayfasını</a> kullanmaktan çekinmeyin.