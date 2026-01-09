A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Domain Yapılandırma Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Bir alan adı, URL değil, örneğin "fastcomments.com" veya "www.example.com". Alt alan sınırlandırması isteniyorsa alt alan adı dahil edilebilir. En fazla 1000 karakter. **/
    domain: string
    /** E-posta gönderiminde kullanılan Gönderici Adı. **/
    emailFromName?: string
    /** E-posta gönderiminde kullanılan Gönderen E-posta adresi. Bu öznitelikte kullanılan alan adına mail.fastcomments.com'un e-posta göndermesine izin verecek şekilde SPF ayarlandığından emin olun. **/
    emailFromEmail?: string
    /** SADECE OKUNUR. Nesnenin oluşturulma zamanı. **/
    createdAt: string
    /** Bu domaine ait logo. E-postalarda kullanılır. HTTPS kullanın. **/
    logoSrc?: string
    /** Bu domaine ait daha küçük logo. HTTPS kullanın. **/
    logoSrc100px?: string
    /** YALNIZCA SSO. Gönderilen her e-postanın altbilgisinde kullanılan URL. "[userId]" değişkenini destekler. **/
    footerUnsubscribeURL?: string
    /** YALNIZCA SSO. Gönderilen her e-postada kullanılan başlıklar. Örneğin teslimatı iyileştirmek için abonelikten çıkma ile ilgili başlıkların ayarlanması için yararlıdır. Bu kayıttaki List-Unsubscribe girdisi, varsa, "[userId]" değişkenini destekler. **/
    emailHeaders?: Record<string, string>
    /** Tüm abonelikten çıkma bağlantılarını devre dışı bırakır. Önerilmez, teslimat oranlarını olumsuz etkileyebilir. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Yapılandırması. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM Yapılandırma Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM kaydınızdaki alan adı. **/
    domainName: string
    /** Kullanılacak DKIM anahtar seçicisi. **/
    keySelector: string
    /** Özel anahtarınız. -----BEGIN PRIVATE KEY----- ile başlamalı ve -----END PRIVATE KEY----- ile bitmelidir **/
    privateKey: string
}
[inline-code-end]

### Kimlik Doğrulama

Alan Yapılandırması, hesabınız için hangi sitelerin FastComments widget'ını barındırabileceğini belirlemek için kullanılır. Bu temel bir kimlik doğrulama biçimidir,
yani herhangi bir Alan Yapılandırması eklenmesi veya kaldırılması, FastComments kurulumunuzun üretimdeki kullanılabilirliğini etkileyebilir.

Halihazırda kullanımda olan bir alan için `Domain Config` içindeki `domain` özelliğini, o alanı devre dışı bırakmak amaçlanmıyorsa, kaldırmayın veya güncellemeyin.

Bu, [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains)'den bir alanı kaldırma ile aynı davranışı gösterir.

Ayrıca, `My Domains` UI'dan bir alanı kaldırmanın, bu UI aracılığıyla eklenmiş olabilecek o alana ait ilgili yapılandırmayı da kaldıracağını unutmayın.

### E-posta Özelleştirmesi İçin

E-posta altbilgisindeki abonelikten çıkma bağlantısı ve birçok e-posta istemcisinin sunduğu tek tıklamayla abonelikten çıkma özelliği, sırasıyla `footerUnsubscribeURL` ve `emailHeaders` tanımlanarak bu API aracılığıyla yapılandırılabilir.

### DKIM İçin

DKIM DNS kayıtlarınızı tanımladıktan sonra, tanımlı yapıyı kullanarak DKIM yapılandırmanızı DomainConfig ile güncellemeniz yeterlidir. 

---