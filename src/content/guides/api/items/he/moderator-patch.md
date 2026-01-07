[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לעדכן `Moderator` לפי `id`.

לעדכון `Moderator` יש את ההגבלות הבאות:

- אין לספק את הערכים הבאים בעת עדכון `Moderator`:
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
- אתה לא יכול לשנות את ה-`tenantId` המשויך ל-`Moderator`.

[inline-code-attrs-start title = 'דוגמת cURL ל-PATCH מנהל תוכן'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH מנהל תוכן'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת PATCH מנהל תוכן'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
