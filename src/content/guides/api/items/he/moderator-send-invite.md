[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

נתיב זה מספק את היכולת להזמין `Moderator` יחיד.

ההגבלות הבאות קיימות לשליחת אימייל הזמנה ל-`Moderator`:
- ה-`Moderator` חייב כבר להתקיים.
- ה-`fromName` לא יכול להיות ארוך מ-`100 תווים`.

**הערות:**
- אם משתמש עם האימייל שסופק כבר קיים, הוא יוזמן למתן את תגובות השוכר שלך.
- אם משתמש עם האימייל שסופק **לא קיים** קישור ההזמנה ינחה אותו דרך יצירת החשבון שלו.
- ההזמנה תפוג לאחר `30 יום`.

אנחנו יכולים ליצור `Moderator` למשתמש שאנחנו מכירים רק את האימייל שלו:

[inline-code-attrs-start title = 'דוגמת cURL להזמנת Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

זה ישלח אימייל כמו `Bob ב-TenantName מזמין אותך להיות מנחה...`

[inline-code-attrs-start title = 'מבנה בקשת הזמנת Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הזמנת Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
