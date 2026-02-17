Objekat `DomainConfig` predstavlja konfiguraciju za domen za tenant.

Struktura objekta `DomainConfig` je sledeća:

[inline-code-attrs-start title = 'Struktura konfiguracije domena'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domen, ne URL, poput "fastcomments.com" ili "www.example.com". Poddomen može biti uključen ako želite ograničiti na poddomen. Maksimalno 1000 karaktera. **/
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

[inline-code-attrs-start title = 'Struktura DKIM konfiguracije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Konfiguracija domena se koristi за одређивање којих sajtova mogu da hostuju FastComments widget за ваш налог. Ovo је базичан облик
аутентификације, што значи да додавање или уклањање било које конфигурације домена може утицати на доступност ваше FastComments инсталације
у продукцији.

Ne uklanjajte ili ažurirajte svojство `domain` objekta `Domain Config` za domen koji је тренутно у употреби, осим ако не намеравате да онемогућите тај домен.

Ovo ima isto понашање као уклањање домена са [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Takođe imajte на уму да уклањање домена из `My Domains` UI ће ukloniti и било коју одговарајућу конфигурацију за тај домен која је можда додата путем тог UI.

### За прилагођавање мејлова

Link za odjavu u podnožju mejla, i функција једноклик одјаве коју нуде многи мејл клијенти, могу се конфигурисати путем овог API-ja дефинисањем `footerUnsubscribeURL` и `emailHeaders`, респективно.

### За DKIM

Након што дефинишете DKIM DNS записе, једноставно ажурирајте DomainConfig са вашом DKIM конфигурацијом користећи дефинисану структуру. 

---