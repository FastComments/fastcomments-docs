Bir `DomainConfig` nesnesi, bir kiracı (tenant) için bir alan adının yapılandırmasını temsil eder.

`DomainConfig` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Alan Adı Yapılandırma Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** "fastcomments.com" veya "www.example.com" gibi bir URL değil, bir alan adı. Yalnızca bir alt alan adıyla sınırlamak isteniyorsa alt alan adı eklenebilir. En fazla 1000 karakter. **/
    domain: string
    /** E-posta gönderilirken kullanılan Gönderen Adı (From-Name). **/
    emailFromName?: string
    /** E-posta gönderilirken kullanılan Gönderen E-postası (From-Email). Bu öznitelikte kullanılan alan adı adına e-posta gönderebilmesi için mail.fastcomments.com'a izin verecek şekilde SPF'nin ayarlandığından emin olun. **/
    emailFromEmail?: string
    /** SALT OKUNUR. Nesnenin oluşturulduğu zaman. **/
    createdAt: string
    /** Bu alan adıyla ilgili logo. E-postalarda kullanılır. HTTPS kullanın. **/
    logoSrc?: string
    /** Bu alan adıyla ilgili daha küçük bir logo. HTTPS kullanın. **/
    logoSrc100px?: string
    /** YALNIZCA SSO. Gönderilen her e-postanın alt bilgisinde kullanılan URL. "[userId]" değişkenini destekler. **/
    footerUnsubscribeURL?: string
    /** YALNIZCA SSO. Gönderilen her e-postada kullanılan başlıklar. Örneğin teslimatı iyileştirmek için abonelikten çıkma ile ilgili başlıkları ayarlamak için kullanışlıdır. Bu Record içindeki List-Unsubscribe girdisi, varsa, "[userId]" değişkenini destekler. **/
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
    /** PEM formatında genel anahtar. GET yanıtlarında döndürülür. **/
    publicKey: string
    /** @deprecated Artık API yanıtlarında döndürülmüyor. Geriye dönük uyumluluk için yazma işlemlerinde kabul edilir. **/
    privateKey?: string
}
[inline-code-end]

### Kimlik Doğrulama İçin

Alan Adı Yapılandırması, hangi sitelerin hesabınız için FastComments widget'ını barındırabileceğini belirlemek için kullanılır. Bu, temel bir kimlik doğrulama biçimidir; yani herhangi bir Alan Adı Yapılandırmasının eklenmesi veya kaldırılması, FastComments kurulumunuzun üretim ortamındaki kullanılabilirliğini etkileyebilir.

Şu anda kullanımda olan bir alan adı için `Domain Config` öğesinin `domain` özelliğini, o alan adını devre dışı bırakma amacı yoksa, kaldırmayın veya güncellemeyin.

Bu, [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) adresinden bir alan adını kaldırmayla aynı davranışa sahiptir.

Ayrıca, `My Domains` arayüzünden bir alan adının kaldırılmasının, bu arayüz aracılığıyla eklenmiş olabilecek o alan adına ait tüm ilgili yapılandırmayı da kaldıracağını unutmayın.

### E-posta Özelleştirme İçin

E-posta alt bilgisindeki abonelikten çıkma bağlantısı ve birçok e-posta istemcisinin sunduğu tek tıkla abonelikten çıkma özelliği, sırasıyla `footerUnsubscribeURL` ve `emailHeaders` tanımlanarak bu API aracılığıyla yapılandırılabilir.

### DKIM İçin

DKIM DNS kayıtlarınızı tanımladıktan sonra, tanımlanan yapıyı kullanarak DomainConfig'i DKIM yapılandırmanızla güncellemeniz yeterlidir.