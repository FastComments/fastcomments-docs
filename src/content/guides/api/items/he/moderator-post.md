[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת להוסיף `Moderator` יחיד.

ליצירת `Moderator` יש את ההגבלות הבאות:

- תמיד יש לספק `name` ו-`email`. `userId` הוא אופציונלי.
- אין לספק את הערכים הבאים בעת יצירת `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- כאשר מצוין `userId`, המשתמש הזה חייב להתקיים.
- כאשר מצוין `userId`, הוא חייב להשתייך לאותו `tenantId` שצוין בפרמטרי השאילתה.
- שני מנהלי תוכן באותו שוכר לא יכולים להתווסף עם אותו `email`.

אנחנו יכולים ליצור `Moderator` למשתמש שאנחנו מכירים רק את האימייל שלו:

[inline-code-attrs-start title = 'דוגמת cURL ליצירת מנהל תוכן באמצעות אימייל'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

או שאנחנו יכולים ליצור `Moderator` למשתמש השייך לשוכר שלנו, כדי לעקוב אחר סטטיסטיקות ניהול התוכן שלו:

[inline-code-attrs-start title = 'דוגמת cURL ליצירת מנהל תוכן באמצעות משתמש שוכר'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת מנהל תוכן'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת מנהל תוכן'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
