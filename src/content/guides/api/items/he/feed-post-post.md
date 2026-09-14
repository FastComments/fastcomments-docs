[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

נתיב זה יוצר `FeedPost` יחיד. לכל פוסט יש מחבר, ולכן `fromUserId` נדרש וחייב להיות המזהה של משתמש FastComments או SSO קיים בחשבון.

[inline-code-attrs-start title = 'דוגמת cURL ליצירת FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** לדחוף את הפוסט לפידים הפתוחים כרגע בדפדפן. ברירת מחדל היא false. **/
    isLive?: boolean
    /** להריץ את הפוסט דרך מנוע הספאם לפני השמירה. ברירת מחדל היא false. **/
    doSpamCheck?: boolean
    /** לדלג על בדיקת תוכן חוזר שמופעלת כחלק מ‑doSpamCheck. ברירת מחדל היא false. **/
    skipDupCheck?: boolean
    /** עד 256 תווים. משודר למאזינים חיים כך שלקוח יכול להתעלם מהשידור שלו עצמו. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** נדרש. מזהה משתמש FastComments או SSO. **/
    fromUserId: string
    title?: string
    /** HTML. מנוקה בעת השמירה. **/
    contentHTML?: string
    /** מתעלם משם התצוגה שנלקח מהמשתמש. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** נכלל במקרה של כשל. **/
    reason?: string
    feedPost?: FeedPost; // אנו מחזירים את הפוסט המלא שנוצר בהצלחה.
}
[inline-code-end]