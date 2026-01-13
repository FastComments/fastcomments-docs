[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

API זה משמש לקבלת תגובות להצגה למשתמש. לדוגמה, הוא מסנן אוטומטית
תגובות לא מאושרות או ספאם.

### עימוד

ניתן לבצע עימוד באחת משתי דרכים, בהתאם לדרישות הביצועים ומקרה השימוש:

1. הכי מהיר: **עימוד מחושב מראש**:
   1. כך FastComments עובד כשאתה משתמש בווידג'טים והלקוחות המוכנים שלנו.
   2. לחיצה על "הבא" פשוט מגדילה את מספר העמוד.
   3. אתה יכול לחשוב על זה כאילו הנתונים מאוחזרים ממאגר מפתח-ערך.
   4. בדרך זו, פשוט הגדר פרמטר `page` שמתחיל ב-`0` וכיוון מיון כ-`direction`.
   5. ניתן להתאים אישית גדלי עמודים באמצעות כללי התאמה אישית.
2. הכי גמיש: **עימוד גמיש**:
   1. בדרך זו אתה יכול להגדיר פרמטרי `limit` ו-`skip` מותאמים אישית. אל תעביר `page`.
   2. כיוון מיון `direction` נתמך גם כן.
   3. `limit` הוא הסך הכל להחזרה לאחר יישום `skip`.
      - דוגמה: הגדר `skip = 200, limit = 100` כאשר `page size = 100` ו-`page = 2`.
   4. תגובות ילד עדיין נספרות בעימוד. אתה יכול לעקוף זאת באמצעות אפשרות `asTree`.
      - אתה יכול לעמד ילדים באמצעות `limitChildren` ו-`skipChildren`.
      - אתה יכול להגביל את עומק השרשורים המוחזרים באמצעות `maxTreeDepth`.

### שרשורים

1. בעת שימוש ב-`עימוד מחושב מראש`, תגובות מקובצות לפי *עמוד* ותגובות בשרשורים משפיעות על העמוד הכולל.
   1. בדרך זו, ניתן לקבוע שרשורים בצד הלקוח בהתבסס על `parentId`.
   2. לדוגמה, עם עמוד עם תגובה אחת ברמה העליונה, ו-29 תשובות, והגדרת `page=0` ב-API - תקבל רק את התגובה ברמה העליונה ו-29 הילדים.
   3. [תמונת דוגמה כאן הממחישה מספר עמודים.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. בעת שימוש ב-`עימוד גמיש`, אתה יכול להגדיר פרמטר `parentId`.
   1. הגדר אותו ל-null כדי לקבל רק תגובות ברמה העליונה.
   2. לאחר מכן כדי לצפות בשרשורים, קרא שוב ל-API והעבר `parentId`.
   3. פתרון נפוץ הוא לבצע קריאת API לתגובות ברמה העליונה ולאחר מכן לבצע קריאות API במקביל לקבלת תגובות לילדים של כל תגובה.
3. __חדש מפברואר 2023!__ אחזר כעץ באמצעות `&asTree=true`.
   1. אתה יכול לחשוב על זה כ-`עימוד גמיש כעץ`.
   2. רק התגובות ברמה העליונה נספרות בעימוד.
   3. הגדר `parentId=null` כדי להתחיל את העץ בשורש (אתה חייב להגדיר `parentId`).
   4. הגדר `skip` ו-`limit` לעימוד.
   5. הגדר `asTree` ל-`true`.
   6. עלות הקרדיטים עולה פי `2x`, מכיוון שהשרת שלנו צריך לעשות הרבה יותר עבודה בתרחיש זה.
   7. הגדר `maxTreeDepth`, `limitChildren`, ו-`skipChildren` כרצונך.

### הסבר עצים

בעת שימוש ב-`asTree`, יכול להיות קשה להבין את העימוד. הנה גרפיקה שימושית:

<div class="screenshot white-bg">
    <div class="title">תרשים עימוד עץ</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="תרשים עימוד עץ" />
</div>

### אחזור תגובות בהקשר של משתמש

ניתן להשתמש ב-API `/comments` בשני הקשרים, למקרי שימוש שונים:

- להחזרת תגובות ממוינות ומתויגות עם מידע לבניית הלקוח שלך.
  - במקרה זה, הגדר פרמטר שאילתה `contextUserId`.
- לאחזור תגובות מהשרת שלך לאינטגרציות מותאמות אישית.
  - הפלטפורמה תעבור לברירת מחדל זו ללא `contextUserId`.

[inline-code-attrs-start title = 'עימוד מחושב מראש לתגובות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש לתגובות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש לתגובות בהקשר משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'עימוד גמיש לתגובות בהקשר משתמש לתגובות ברמה העליונה בלבד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### קבלת תגובות כעץ

ניתן לקבל את התגובות מוחזרות כעץ, כאשר העימוד סופר רק את התגובות ברמה העליונה.

[inline-code-attrs-start title = 'תגובות כעץ בהקשר משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

רוצה לקבל רק את התגובות ברמה העליונה והילדים המיידיים? הנה דרך אחת:

[inline-code-attrs-start title = 'תגובות כעץ עם עומק מקסימלי'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

עם זאת, בממשק המשתמש שלך ייתכן שתצטרך לדעת האם להציג כפתור "הצג תשובות" על
כל תגובה. בעת אחזור תגובות באמצעות עץ יש מאפיין `hasChildren` המתויג
על תגובות כאשר רלוונטי.

### קבלת תגובות כעץ, חיפוש לפי האשטאג

ניתן לחפש לפי האשטאג באמצעות ה-API, בכל השוכר שלך (לא מוגבל לעמוד אחד, או `urlId`).

בדוגמה זו, אנחנו משמיטים `urlId`, ומחפשים לפי מספר האשטאגים. ה-API יחזיר רק תגובות שיש להן את כל ההאשטאגים המבוקשים.

[inline-code-attrs-start title = 'תגובות כעץ בהקשר משתמש, לפי האשטאג'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

### התגובה

[inline-code-attrs-start title = 'מבנה תגובת תגובות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### טיפים מועילים

#### URL ID

סביר להניח שתרצה להשתמש ב-API של `Comment` עם פרמטר `urlId`. אתה יכול לקרוא קודם ל-API של `Pages`, כדי לראות איך נראים ערכי `urlId` הזמינים לך.

#### פעולות אנונימיות

לתגובות אנונימיות סביר להניח שתרצה להעביר `anonUserId` בעת אחזור תגובות, ובעת ביצוע דיווח וחסימה.

(!) זה נדרש עבור חנויות אפליקציות רבות מכיוון שמשתמשים חייבים להיות מסוגלים לדווח על תוכן שנוצר על ידי משתמשים שהם יכולים לראות, גם אם הם לא מחוברים. אי עשיית זאת עלולה לגרום להסרת האפליקציה שלך מהחנות האמורה.

#### תגובות לא מוחזרות

בדוק שהתגובות שלך מאושרות, ואינן ספאם.
