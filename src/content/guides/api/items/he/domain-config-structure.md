אובייקט `DomainConfig` מייצג תצורה עבור דומיין של שוכר.

המבנה של האובייקט `DomainConfig` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה תצורת הדומיין'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** דומיין, לא URL, כמו "fastcomments.com" או "www.example.com". ניתן לכלול תת‑דומיין אם רוצים להגביל לתת‑דומיין. מקסימום 1000 תווים. **/
    domain: string
    /** השם שמופיע בשדה From בעת שליחת אימיילים. **/
    emailFromName?: string
    /** כתובת ה-From המושמשת לשליחת אימיילים. ודאו ש-SPF מוגדר כדי לאפשר ל-mail.fastcomments.com לשלוח אימיילים בשם הדומיין המופיע בתכונה זו. **/
    emailFromEmail?: string
    /** לקריאה בלבד. מועד יצירת האובייקט. **/
    createdAt: string
    /** הלוגו הקשור לדומיין זה. משמש באימיילים. השתמשו ב-HTTPS. **/
    logoSrc?: string
    /** לוגו קטן יותר הקשור לדומיין זה. השתמשו ב-HTTPS. **/
    logoSrc100px?: string
    /** SSO בלבד. ה-URL המשמש בכותרת תחתונה של כל אימייל שנשלח. תומך במשתנה "[userId]". **/
    footerUnsubscribeURL?: string
    /** SSO בלבד. הכותרות שבהן משתמשים בכל אימייל שנשלח. שימושי, למשל, להגדרת כותרות הקשורות להסרה מרשימת תפוצה לשיפור מסירה. הערך List-Unsubscribe ברשומה זו, אם קיים, תומך במשתנה "[userId]". **/
    emailHeaders?: Record<string, string>
    /** השבתת כל קישורי הסרה מרשימת תפוצה. לא מומלץ, עלול לפגוע בשיעורי המסירה. **/
    disableUnsubscribeLinks?: boolean
    /** תצורת DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תצורת DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** שם הדומיין ברשומת DKIM שלך. **/
    domainName: string
    /** מצביע המפתח (selector) של DKIM לשימוש. **/
    keySelector: string
    /** המפתח הציבורי בפורמט PEM. מוחזר בתגובות GET. **/
    publicKey: string
    /** @deprecated לא מוחזר עוד בתגובות ה-API. מתקבל בכתיבה עבור תאימות לאחור. **/
    privateKey?: string
}
[inline-code-end]

### לצורך אימות

תצורת דומיינים משמשת לקביעת אילו אתרים יכולים לארח את הווידג'ט של FastComments עבור החשבון שלך. זוהי צורה בסיסית של
אימות, כלומר הוספה או הסרה של תצורות דומיין עלולה להשפיע על זמינות התקנת FastComments שלך
בסביבת הייצור.

אל תסירו או תעדכנו את המאפיין `domain` של `Domain Config` עבור דומיין שנמצא בשימוש כרגע אלא אם המטרה היא לנטרל את הדומיין.

זה מתנהג באותו אופן כמו הסרת דומיין מ-[/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

שימו לב שגם הסרה של דומיין מ-`My Domains` UI תסיר כל תצורה מקבילה עבור אותו דומיין שעשויה להיות נוספה דרך ממשק זה.

### להתאמה אישית של אימיילים

קישור ההסרה בכותרת התחתונה של המייל, ותכונת 'הסרה בלחיצה אחת' שמוצעת על ידי לקוחות דוא״ל רבים, ניתנים להגדרה דרך ה-API הזה על ידי הגדרת `footerUnsubscribeURL` ו-`emailHeaders`, בהתאמה.

### לגבי DKIM

לאחר שהגדרתם את רשומות ה-DNS של DKIM שלכם, פשוט עדכנו את DomainConfig עם תצורת DKIM שלכם באמצעות המבנה המוגדר.

---