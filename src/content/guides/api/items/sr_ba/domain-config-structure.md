---
A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Структура конфигурације домена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, не УРЛ, као на пример "fastcomments.com" или "www.example.com". Поддомен може бити укључен ако желите ограничити на поддомен. Максимално 1000 знакова. **/
    domain: string
    /** Име пошиљаоца (From-Name) које се користи при слању имејлова. **/
    emailFromName?: string
    /** Адреса пошиљаоца (From-Email) која се користи при слању имејлова. Осигурајте да је SPF подешен да дозволи mail.fastcomments.com да шаље имејлове у име домена коришћеног у овом атрибуту. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** Лого повезан са овим доменом. Користи се у имејловима. Користите HTTPS. **/
    logoSrc?: string
    /** Мањи лого повезан са овим доменом. Користите HTTPS. **/
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

[inline-code-attrs-start title = 'Структура DKIM конфигурације'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### За аутентификацију

Конфигурација домена се користи да утврди који сајтови могу угостити FastComments виџет за ваш налог. Ово је основни облик аутентификације, што значи да додавање или уклањање било које конфигурације домена може утицати на доступност ваше FastComments инсталације у продукцији.

Не уклањајте или ажурирајте својство `domain` у `Domain Config` за домен који је тренутно у употреби осим ако није намера да се тај домен онемогући.

Ово има исто понашање као уклањање домена са [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Такође, имајте у виду да уклањање домена из `My Domains` UI уклониће сваку одговарајућу конфигурацију за тај домен која је можда додата преко тог интерфејса.

### За прилагођавање имејлова

Линк за одјаву у фоотеру имејла, и функција једним кликом за одјаву коју нуде многи имејл клијенти, могу се конфигурисати преко овог API-ја дефинисањем `footerUnsubscribeURL` и `emailHeaders`, респективно.

### За DKIM

Након дефинисања ваших DKIM DNS записа, једноставно ажурирајте DomainConfig са вашом DKIM конфигурацијом користећи дефинисану структуру. 

---