[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

API זה משמש לקבלת תגובות להצגה למשתמש. לדוגמה, הוא מסנן אוטומטית
תגובות שלא אושרו או שסומנו כספאם.

### עימוד (Pagination)

ניתן לבצע עימוד באחת משתי דרכים, בהתאם לדרישות הביצועים ולתרחיש השימוש:

1. המהיר ביותר: **עימוד מחושב מראש (Precalculated Pagination)**:
   1. כך FastComments עובד כאשר אתם משתמשים בווידג'טים ובלקוחות המוכנים שלנו.
   2. לחיצה על "הבא" פשוט מגדילה את מספר העמוד.
   3. ניתן לחשוב על כך כאילו הנתונים נשלפים ממאגר מפתח-ערך (key-value store).
   4. בדרך זו, פשוט הגדירו פרמטר `page` החל מ-`0` וכיוון מיון כ-`direction`.
   5. ניתן להתאים אישית את גודל העמוד באמצעות כללי התאמה אישית.
2. הגמיש ביותר: **עימוד גמיש (Flexible Pagination)**:
   1. בדרך זו תוכלו להגדיר פרמטרים מותאמים אישית `limit` ו-`skip`. אל תעבירו `page`.
   2. כיוון מיון `direction` נתמך גם כן.
   3. `limit` הוא המספר הכולל של תגובות להחזרה לאחר החלת `skip`.
      - דוגמה: הגדירו `skip = 200, limit = 100` כאשר `page size = 100` ו-`page = 2`.
   4. תגובות בנות עדיין נספרות בעימוד. ניתן לעקוף זאת באמצעות האפשרות `asTree`.
      - ניתן לעמדד תגובות בנות באמצעות `limitChildren` ו-`skipChildren`.
      - ניתן להגביל את העומק של השרשורים המוחזרים באמצעות `maxTreeDepth`.

### שרשורים (Threads)

1. בעת שימוש ב-`Precalculated Pagination`, תגובות מקובצות לפי *עמוד* ותגובות בשרשורים משפיעות על העמוד הכללי.
   1. בדרך זו, שרשורים יכולים להיקבע בצד הלקוח על בסיס `parentId`.
   2. לדוגמה, עבור עמוד עם תגובה אחת ברמה העליונה ו-29 תשובות, והגדרת `page=0` ב-API - תקבלו רק את התגובה ברמה העליונה ואת 29 התגובות הבנות.
2. בעת שימוש ב-`Flexible Pagination`, תוכלו להגדיר פרמטר `parentId`.
   1. הגדירו זאת ל-null כדי לקבל רק תגובות ברמה העליונה.
   2. ואז, כדי לצפות בשרשורים, קראו ל-API שוב והעבירו `parentId`.
   3. פתרון נפוץ הוא לבצע קריאת API לתגובות ברמה העליונה ואז לבצע קריאות API מקבילות לקבלת תגובות עבור הבנים של כל תגובה.
3. __חדש החל מפברואר 2023!__ שליפה כעץ באמצעות `&asTree=true`.
   1. ניתן לחשוב על כך כ-`Flexible Pagination as a Tree`.
   2. רק תגובות ברמה העליונה נספרות בעימוד.
   3. הגדירו `parentId=null` כדי להתחיל את העץ מהשורש (חובה להגדיר `parentId`).
   4. הגדירו `skip` ו-`limit` לעימוד.
   5. הגדירו `asTree` ל-`true`.
   6. עלות הקרדיטים עולה פי `2x`, מכיוון שה-backend שלנו צריך לבצע הרבה יותר עבודה בתרחיש זה.
   7. הגדירו `maxTreeDepth`, `limitChildren` ו-`skipChildren` לפי הצורך.

### הסבר על עצים (Trees)

בעת שימוש ב-`asTree`, יכול להיות קשה להבין את העימוד. הנה גרפיקה שימושית:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### שליפת תגובות בהקשר של משתמש

ניתן להשתמש ב-API של `/comments` בשני הקשרים, לתרחישי שימוש שונים:

- להחזרת תגובות ממוינות ומתויגות עם מידע לבניית לקוח משלכם.
  - במקרה זה, הגדירו פרמטר שאילתא `contextUserId`.
- לשליפת תגובות מה-backend שלכם לאינטגרציות מותאמות אישית.
  - הפלטפורמה תשתמש בברירת מחדל זו ללא `contextUserId`.

[inline-code-attrs-start title = 'עימוד מחושב מראש של תגובות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש של תגובות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש של תגובות בהקשר משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש של תגובות בהקשר משתמש לתגובות ברמה העליונה בלבד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### קבלת תגובות כעץ

ניתן לקבל את התגובות כעץ, כאשר העימוד סופר רק את התגובות ברמה העליונה.

[inline-code-attrs-start title = 'תגובות כעץ בהקשר משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

רוצים לקבל רק את התגובות ברמה העליונה ואת הבנים המיידיים? הנו דרך אחת:

[inline-code-attrs-start title = 'תגובות כעץ עם עומק מקסימלי'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

עם זאת, בממשק המשתמש שלכם ייתכן שתצטרכו לדעת האם להציג כפתור "הצג תשובות" על
כל תגובה. בעת שליפת תגובות כעץ קיימת מאפיין `hasChildren` שמתווסף
לתגובות כאשר רלוונטי.

### קבלת תגובות כעץ, חיפוש לפי האשטג

ניתן לחפש לפי האשטג באמצעות ה-API, על פני כל ה-tenant שלכם (לא מוגבל לעמוד אחד, או ל-`urlId`).

בדוגמה זו, אנו משמיטים את `urlId`, ומחפשים לפי מספר האשטגים. ה-API יחזיר רק תגובות שמכילות את כל ההאשטגים המבוקשים.

[inline-code-attrs-start title = 'תגובות כעץ בהקשר משתמש, לפי האשטג'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### כל פרמטרי הבקשה

[inline-code-attrs-start title = 'מבנה בקשת תגובות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### התשובה

[inline-code-attrs-start title = 'מבנה תשובת תגובות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### טיפים שימושיים

#### מזהה URL

סביר להניח שתרצו להשתמש ב-API של `Comment` עם הפרמטר `urlId`. תוכלו לקרוא קודם ל-API של `Pages`, כדי לראות כיצד נראים ערכי `urlId` הזמינים לכם.

#### פעולות אנונימיות

עבור תגובות אנונימיות, סביר להניח שתרצו להעביר `anonUserId` בעת שליפת תגובות, ובעת ביצוע סימון (flagging) וחסימה (blocking).

(!) זה נדרש עבור חנויות אפליקציות רבות, מכיוון שמשתמשים חייבים להיות מסוגלים לסמן תוכן שנוצר על ידי משתמשים שהם רואים, גם אם הם אינם מחוברים. אי ביצוע זה עלול לגרום להסרת האפליקציה שלכם מהחנות.

#### תגובות שאינן מוחזרות

בדקו שהתגובות שלכם אושרו, ושאינן ספאם.