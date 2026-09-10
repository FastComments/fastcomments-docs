[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

מתאר את האישור המבצע את הבקשה: השוכר שאליו הוא שייך, ובמקרה של אסימוני גישה של OAuth, המשתמש שאישר את היישום. אינטגרציות משתמשות בו כדי לבדוק חיבור ולתייג אותו.

עם מפתח API, התגובה מזהה רק את השוכר. עם אסימון נושא של OAuth היא גם נושאת את המשתמש המורשה ואת ההיקפים שהוענקו.

[inline-code-attrs-start title = 'דוגמת cURL של Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]