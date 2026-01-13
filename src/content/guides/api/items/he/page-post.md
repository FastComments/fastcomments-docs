[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת ליצור עמודים.

מקרה שימוש נפוץ הוא בקרת גישה.

הערות:

- אם הגבת בשרשור תגובות, או קראת ל-API ליצירת `Comment`, כבר יצרת אובייקט `Page`! אתה יכול לנסות לאחזר אותו דרך
  נתיב `/by-url-id` של `Page`, תוך העברת אותו `urlId` שהועבר לווידג'ט התגובות.
- מבנה ה-`Page` מכיל כמה ערכים **מחושבים**.
  כרגע, אלה הם `commentCount` ו-`rootCommentCount`.
  הם מאוכלסים אוטומטית ולא ניתן להגדיר אותם על ידי ה-API. ניסיון לעשות זאת יגרום ל-API להחזיר שגיאה.

[inline-code-attrs-start title = 'דוגמת cURL ל-POST Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת POST Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
