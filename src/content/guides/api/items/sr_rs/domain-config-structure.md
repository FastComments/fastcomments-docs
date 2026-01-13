A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Структура конфигурације домена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, не URL, као што је "fastcomments.com" или "www.example.com". Поддомен може бити укључен ако желите ограничити на поддомен. Максимално 1000 карактера. **/
    domain: string
    /** Име пошиљаоца које се користи при слању имејлова. **/
    emailFromName?: string
    /** From-Email који се користи при слању имејлова. Осигурајте да је SPF подешен да омогући mail.fastcomments.com да шаље имејлове у име домена који се користи у овом атрибуту. **/
    emailFromEmail?: string
    /** Само за читање. Када је објекат креиран. **/
    createdAt: string
    /** Лого везан за овај домен. Користи се у имејловима. Користите HTTPS. **/
    logoSrc?: string
    /** Мањи лого везан за овај домен. Користите HTTPS. **/
    logoSrc100px?: string
    /** Само за SSO. URL који се користи у фоотеру сваког послатог имејла. Подржава променљиву "[userId]". **/
    footerUnsubscribeURL?: string
    /** Само за SSO. Хедери који се користе у сваком послатом имејлу. Корисно, на пример, за подешавање хедера везаних за отписивање ради побољшања испоруке. Ставка List-Unsubscribe у овом запису, ако постоји, подржава променљиву "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Онемогући све линкове за отписивање. Није препоручљиво, може негативно утицати на стопе испоруке. **/
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
    /** Селектор DKIM кључа који треба користити. **/
    keySelector: string
    /** Ваш приватни кључ. Почните са -----BEGIN PRIVATE KEY----- и завршите са -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### За аутентификацију

Конфигурација домена се користи да одреди који сајтови могу угостити FastComments видгет за ваш налог. Ово је основни облик
аутентификације, што значи да додавање или уклањање било којих конфигурација домена може утицати на доступност ваше FastComments инсталације
у продукцији.

Не уклањајте или мењајте својство `domain` у `Domain Config` за домен који се тренутно користи осим ако заиста не намеравате да онемогућите тај домен.

Ово има исто понашање као уклањање домена из [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Такође имајте у виду да ће уклањање домена из интерфејса `My Domains` уклонити и сваку одговарајућу конфигурацију за тај домен која је можда додата преко тог интерфејса.

### За прилагођавање имејлова

Линк за отписивање у фоотеру имејла, и функција једним кликом за отписивање коју нуде многи имејл клијенти, могу се подесити преко овог API-ја дефинисањем `footerUnsubscribeURL` и `emailHeaders`, респективно.

### За DKIM

Након дефинисања ваших DKIM DNS записа, једноставно ажурирајте DomainConfig са вашом DKIM конфигурацијом користећи дефинисану структуру. 

---