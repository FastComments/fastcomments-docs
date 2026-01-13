אובייקט `EmailTemplate` מייצג קונפיגורציה לתבנית אימייל מותאמת אישית, עבור שוכר.

המערכת תבחר את תבנית האימייל לשימוש באמצעות:

- מזהה הסוג שלה, אנחנו קוראים לזה `emailTemplateId`. אלה קבועים.
- ה-`domain`. אנחנו קודם ננסה למצוא תבנית עבור הדומיין שהאובייקט הקשור (כמו `Comment`) קשור אליו, ואם לא נמצאה התאמה אז ננסה למצוא תבנית שבה domain הוא null או `*`.

המבנה עבור אובייקט `EmailTemplate` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה תבנית אימייל'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### הערות

- אתה יכול לקבל את ערכי `emailTemplateId` החוקיים מנקודת הקצה `/definitions`.
- נקודת הקצה `/definitions` כוללת גם את התרגומים ברירת המחדל ונתוני הבדיקה.
- תבניות ייכשלו בשמירה עם מבנה או נתוני בדיקה לא חוקיים.
