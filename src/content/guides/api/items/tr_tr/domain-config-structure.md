A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Domain Yapılandırma Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Bir alan adı, URL değil, örneğin "fastcomments.com" veya "www.example.com". Alt alan adı, alt alan adına sınırlama istenirse dahil edilebilir. Maksimum 1000 karakter. **/
    domain: string
    /** E-posta gönderilirken kullanılan Gönderen Adı. **/
    emailFromName?: string
    /** E-postalar gönderilirken kullanılan Gönderen E-posta. Bu öznitelikte kullanılan alan adına mail.fastcomments.com'ın e-posta göndermesine izin vermek için SPF'in yapılandırıldığından emin olun. **/
    emailFromEmail?: string
    /** SADECE OKUNUR. Nesnenin ne zaman oluşturulduğu. **/
    createdAt: string
    /** Bu alan adıyla ilişkili logo. E-postalarda kullanılır. HTTPS kullanın. **/
    logoSrc?: string
    /** Bu alan adıyla ilişkili daha küçük bir logo. HTTPS kullanın. **/
    logoSrc100px?: string
    /** SADECE SSO. Gönderilen her e-postanın altbilgisinde kullanılan URL. "[userId]" değişkenini destekler. **/
    footerUnsubscribeURL?: string
    /** SADECE SSO. Gönderilen her e-postada kullanılan başlıklar. Örneğin teslimatı iyileştirmek için abonelikten çıkma ile ilgili başlıkları ayarlamak için yararlıdır. Bu Kaydaki List-Unsubscribe girişi, mevcutsa, "[userId]" değişkenini destekler. **/
    emailHeaders?: Record<string, string>
    /** Tüm abonelikten çıkma bağlantılarını devre dışı bırakır. Önerilmez, teslimat oranlarına zarar verebilir. **/
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
    /** PEM formatında açık anahtar. GET yanıtlarında döndürülür. **/
    publicKey: string
    /** @deprecated Artık API yanıtlarında döndürülmez. Geriye dönük uyumluluk için yazmada kabul edilir. **/
    privateKey?: string
}
[inline-code-end]

### Kimlik Doğrulama

Domain Configuration is used to determine which sites can host the FastComments widget for your account. This is a basic form
of authentication, meaning adding or removing any Domain Configurations can impact the availability of your FastComments installation
in production.

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### E-posta Özelleştirmesi İçin

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### DKIM İçin

After defining your DKIM DNS records, simply update the DomainConfig with your DKIM configuration using the defined structure. 

---