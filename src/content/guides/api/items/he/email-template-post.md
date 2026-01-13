[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת ליצור תבניות אימייל.

הערות:

- לא יכולות להיות מספר תבניות עם אותו `emailTemplateId` עם אותו דומיין.
- אבל אתה יכול לקבל תבנית עם תו כללי (`domain` = `*` ותבנית ספציפית לדומיין לאותו `emailTemplateId`).
- ציון `domain` רלוונטי רק אם יש לך דומיינים שונים, או רוצה להשתמש בתבניות ספציפיות לבדיקות (`domain` מוגדר ל-`localhost` וכד').
- אם אתה כן מציין `domain` הוא חייב להתאים ל-`DomainConfig`. בשגיאה מסופקת רשימה של דומיינים חוקיים.
- תחביר התבנית הוא EJS והיא מרונדרת עם timeout של 500ms. P99 לרנדור הוא <5ms, אז אם אתה מגיע ל-500ms משהו לא בסדר.
- **התבנית שלך חייבת להירנדר עם ה-`testData` הנתון** כדי להישמר. שגיאות רנדור נאספות ומדווחות בדשבורד (בקרוב יהיה זמין דרך API).

הנתונים המינימליים הנדרשים להוספת תבנית הם כדלקמן:

[inline-code-attrs-start title = 'דוגמת cURL מינימלית ל-POST תבנית אימייל'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

ייתכן שתרצה תבניות לכל אתר, במקרה זה אתה מגדיר `domain`:

[inline-code-attrs-start title = 'דוגמת cURL ל-POST תבנית אימייל'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת POST תבנית אימייל'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת POST תבנית אימייל'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
