Об'єкт `DomainConfig` представляє конфігурацію для домену для орендаря.

Структура об'єкта `DomainConfig` виглядає так:

[inline-code-attrs-start title = 'Структура конфігурації домену'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, а не URL, наприклад "fastcomments.com" або "www.example.com". Можна вказати піддомен, якщо потрібно обмежити до піддомену. Максимум 1000 символів. **/
    domain: string
    /** Ім'я відправника, яке використовується при надсиланні електронних листів. **/
    emailFromName?: string
    /** Адреса електронної пошти відправника (From-Email), яка використовується при надсиланні листів. Переконайтеся, що SPF налаштовано, щоб дозволити mail.fastcomments.com відправляти листи від імені домену, вказаного в цьому атрибуті. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** Логотип, пов'язаний із цим доменом. Використовується в листах. Використовуйте HTTPS. **/
    logoSrc?: string
    /** Менший логотип, пов'язаний із цим доменом. Використовуйте HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура конфігурації DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** The public key, in PEM format. Returned in GET responses. **/
    publicKey: string
    /** @deprecated No longer returned in API responses. Accepted on write for backwards compatibility. **/
    privateKey?: string
}
[inline-code-end]

### Для автентифікації

Domain Configuration is used to determine which sites can host the FastComments widget for your account. This is a basic form
of authentication, meaning adding or removing any Domain Configurations can impact the availability of your FastComments installation
in production.

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### Для налаштування електронної пошти

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### Для DKIM

After defining your DKIM DNS records, simply update the DomainConfig with your DKIM configuration using the defined structure. 

---