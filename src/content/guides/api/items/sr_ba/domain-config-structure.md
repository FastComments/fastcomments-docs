Објекат `DomainConfig` представља конфигурацију домена за тенанта.

Структура објекта `DomainConfig` је следећа:

[inline-code-attrs-start title = 'Структура DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, не URL, као што су "fastcomments.com" или "www.example.com". Може се укључити поддомен ако желите ограничити на поддомен. Максимум 1000 карактера. **/
    domain: string
    /** Име пошиљаоца које се користи при слању е-поште. **/
    emailFromName?: string
    /** From-Email који се користи при слању е-поште. Осигурајте да је SPF подешен тако да mail.fastcomments.com може слати мејлове у име домена коришћеног у овом атрибуту. **/
    emailFromEmail?: string
    /** Само за читање. Када је објекат креиран. **/
    createdAt: string
    /** Лого повезан са овим доменом. Користи се у е-порукама. Користите HTTPS. **/
    logoSrc?: string
    /** Мањи лого повезан са овим доменом. Користите HTTPS. **/
    logoSrc100px?: string
    /** SAMO за SSO. URL који се користи у подножју сваког послатог мејла. Подржава променљиву "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO за SSO. Хедери који се користе у сваком послатом мејлу. Корисно, на пример, за подешавање хедера повезаних са отписивањем ради побољшања испоруке. Унос List-Unsubscribe у овом запису, ако постоји, подржава променљиву "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Онемогућити све линкове за отписивање. Не препоручује се, може негативно утицати на стопе испоруке. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM конфигурација. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура DKIM конфигурације'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Име домена у вашем DKIM запису. **/
    domainName: string
    /** Селектор DKIM кључа који ће се користити. **/
    keySelector: string
    /** Ваш приватни кључ. Почinje са -----BEGIN PRIVATE KEY----- и завршава са -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### За аутентификацију

Конфигурација домена се користи за одређивање који сајтови могу да хостују FastComments видгет за ваш налог. Ово је основни облик
аутентификације, што значи да додавање или уклањање било које конфигурације домена може утицати на доступност ваше FastComments инсталације
у продукцији.

Не уклањајте и не ажурирајте својство `domain` објекта `Domain Config` за домен који је тренутно у употреби, осим ако није намера да се тај домен онемогући.

Ово има исто понашање као уклањање домена из [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Такође имајте на уму да уклањање домена из `My Domains` корисничког интерфејса уклониће било коју одговарајућу конфигурацију за тај домен која је можда додата преко тог интерфејса.

### За прилагођавање е-поште

Линк за отписивање у подножју е-поруке и функција једнокликовног отписивања коју нуди много клијената за електронску пошту могу се конфигурисати преко овог API-ја дефинисањем `footerUnsubscribeURL` и `emailHeaders`, редом.

### За DKIM

Након дефинисања ваших DKIM DNS записа, једноставно ажурирајте DomainConfig са вашом DKIM конфигурацијом користећи дефинисану структуру. 

---