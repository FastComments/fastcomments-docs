Обектът `DomainConfig` представлява конфигурация за домейн за tenant.

Структурата на обекта `DomainConfig` е следната:

[inline-code-attrs-start title = 'Структура на Domain Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
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

[inline-code-attrs-start title = 'Структура на DKIM Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### За удостоверяване

Конфигурацията на домейна се използва за определяне кои сайтове могат да хостват уиджета FastComments за вашия акаунт. Това е основна форма
на удостоверяване, което означава, че добавянето или премахването на каквито и да е конфигурации на домейни може да повлияе на наличността на вашата инсталация на FastComments
в продукция.

Не премахвайте или актуализирайте свойството `domain` на `Domain Config` за домейн, който в момента се използва, освен ако деактивирането на този домейн е предвидено.

Това има същото поведение като премахването на домейн от [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Също така имайте предвид, че премахването на домейн от потребителския интерфейс `My Domains` ще премахне всяка съответна конфигурация за този домейн, която може да е била добавена чрез този потребителски интерфейс.

### За персонализация на имейли

Връзката за отписване в долния колонтитул на имейла и функцията за отписване с едно щракване, предлагана от много имейл клиенти, могат да бъдат конфигурирани чрез този API, като дефинирате съответно `footerUnsubscribeURL` и `emailHeaders`.

### За DKIM

След като дефинирате вашите DKIM DNS записи, просто актуализирайте DomainConfig с вашата DKIM конфигурация, като използвате дефинираната структура.
