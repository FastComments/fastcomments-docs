Објекат `DomainConfig` представља конфигурацију за домен за тенанта.

Структура објекта `DomainConfig` је следећа:

[inline-code-attrs-start title = 'Структура конфигурације домена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, не URL, као на пример "fastcomments.com" или "www.example.com". Поддомен може бити укључен ако се жели ограничити на поддомен. Максимално 1000 карактера. **/
    domain: string
    /** Име пошиљаоца које се користи при слању имејлова. **/
    emailFromName?: string
    /** From-Email који се користи при слању имејлова. Осигурајте да је SPF подешен како би се дозволило mail.fastcomments.com да шаље имејлове као домен наведен у овом пољу. **/
    emailFromEmail?: string
    /** САМО ЗА ЧИТАЊЕ. Када је објекат креиран. **/
    createdAt: string
    /** Лого повезан са овим доменом. Користи се у имејловима. Користите HTTPS. **/
    logoSrc?: string
    /** Мањи лого повезан са овим доменом. Користите HTTPS. **/
    logoSrc100px?: string
    /** Само за SSO. URL који се користи у фудеру сваког послатог имејла. Подржава променљиву "[userId]". **/
    footerUnsubscribeURL?: string
    /** Само за SSO. Заглавља која се користе у сваком послатом имејлу. Корисно, на пример, за подешавање заглавља везаних за одјаву како би се побољшала испоручивост. Ентрија List-Unsubscribe у овом запису, ако постоји, подржава променљиву "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Онемогућити све линкове за одјаву. Непрепоручљиво, може погоршати стопе испоруке. **/
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
    /** Јавни кључ у PEM формату. Враћа се у GET одговорима. **/
    publicKey: string
    /** @deprecated Више се не враћа у API одговорима. Прихвата се при запису ради назадне компатибилности. **/
    privateKey?: string
}
[inline-code-end]

### За аутентификацију

Конфигурација домена се користи за одређивање који сајтови могу хостовати FastComments видгет за ваш налог. Ово је основни облик аутентикације, што значи да додавање или уклањање било којих конфигурација домена може утицати на доступност ваше FastComments инсталације у продукцији.

Не уклањајте нити ажурирајте својство `domain` у `Domain Config` за домен који је тренутно у употреби, осим ако није намерно онемогућавање тог домена.

Ово има исто понашање као уклањање домена са [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Напомените и да ће уклањање домена из UI-а `My Domains` уклонити и сваку одговарајућу конфигурацију за тај домен која је можда додата преко тог UI-а.

### За прилагођавање имејлова

Линк за одјаву у фудеру имејла и функција једнокликовске одјаве коју нуде многи клијенти за имејлове могу се конфигурисати преко овог API-ја дефинисањем `footerUnsubscribeURL` односно `emailHeaders`.

### За DKIM

Након дефинисања ваших DKIM DNS записа, једноставно ажурирајте DomainConfig са вашом DKIM конфигурацијом користећи дефинисану структуру.

---