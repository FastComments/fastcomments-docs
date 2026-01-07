אובייקט `DomainConfig` מייצג קונפיגורציה עבור דומיין עבור שוכר.

המבנה עבור אובייקט `DomainConfig` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה קונפיגורציית דומיין'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'מבנה קונפיגורציית DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### לאימות

קונפיגורציית דומיין משמשת לקביעת אילו אתרים יכולים לארח את ווידג'ט FastComments עבור החשבון שלך. זו צורה בסיסית
של אימות, כלומר הוספה או הסרה של קונפיגורציות דומיין יכולה להשפיע על זמינות התקנת FastComments שלך
בייצור.

אל תסיר או תעדכן את מאפיין `domain` של `Domain Config` עבור דומיין שנמצא בשימוש כרגע אלא אם כן השבתת אותו דומיין היא מכוונת.

להתנהגות זו יש את אותה התנהגות כמו הסרת דומיין מ-[/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

שים לב גם שהסרת דומיין מממשק המשתמש `הדומיינים שלי` תסיר כל קונפיגורציה תואמת לאותו דומיין שאולי נוספה דרך ממשק משתמש זה.

### להתאמה אישית של אימייל

קישור ביטול ההרשמה בתחתית האימייל, ותכונת ביטול ההרשמה בלחיצה אחת המוצעת על ידי לקוחות אימייל רבים, ניתנות להגדרה דרך API זה על ידי הגדרת `footerUnsubscribeURL` ו-`emailHeaders`, בהתאמה.

### עבור DKIM

לאחר הגדרת רשומות ה-DNS של DKIM שלך, פשוט עדכן את DomainConfig עם קונפיגורציית ה-DKIM שלך באמצעות המבנה המוגדר.
