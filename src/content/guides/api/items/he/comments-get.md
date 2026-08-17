[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

API זה משמש לקבלת תגובות להצגה למשתמש. לדוגמה, הוא מסנן אוטומטית תגובות שלא אושרו או תגובות ספאם.

### Pagination

ניתן לבצע דפדוף באחת משתי דרכים, בהתאם לדרישות הביצועים ולמקרה השימוש:

1. **הכי מהיר: Precalculated Pagination**:
   1. זהו האופן שבו FastComments פועל כאשר אתה משתמש בווידג'טים ובקלאיינטים המוכנים שלנו.
   2. לחיצה על "הבא" פשוט מגדילה את מספר העמוד.
   3. ניתן לחשוב על זה כעל שליפה ממאגר מפתח-ערך.
   4. בצורה זו, פשוט הגדר פרמטר `page` שמתחיל ב-`0` וכיוון מיון כ-`direction`.
   5. ניתן להתאים את גודל העמודים באמצעות כללי התאמה.
2. **הכי גמיש: Flexible Pagination**:
   1. בצורה זו ניתן להגדיר פרמטרים מותאמים `limit` ו-`skip`. אל תעבירו `page`.
   2. כיוון המיון `direction` נתמך גם הוא.
   3. `limit` הוא המספר הכולל שיש להחזיר לאחר יישום `skip`.
      - לדוגמה: הגדר `skip = 200, limit = 100` כאשר `page size = 100` ו-`page = 2`.
   4. תגובות ילדים עדיין נספרות בדפדוף. ניתן לעקוף זאת באמצעות האפשרות `asTree`.
      - ניתן לדפדף תגובות ילדים באמצעות `limitChildren` ו-`skipChildren`.
      - ניתן להגביל את עומק השרשורים המוחזרים באמצעות `maxTreeDepth`.

### Threads

1. כאשר משתמשים ב-`Precalculated Pagination`, תגובות מקובצות לפי *page* והתגובות בשרשורים משפיעות על העמוד הכולל.
   1. בצורה זו, ניתן לקבוע שרשורים בצד הלקוח בהתבסס על `parentId`.
   2. לדוגמה, בעמוד עם תגובה ברמה העליונה אחת ו-29 תגובות, והגדרת `page=0` ב-API - תקבל רק את התגובה ברמה העליונה ואת 29 הילדים.
2. כאשר משתמשים ב-`Flexible Pagination`, ניתן להגדיר פרמטר `parentId`.
   1. הגדר זאת ל-null כדי לקבל רק תגובות ברמה העליונה.
   2. לאחר מכן, כדי לצפות בשרשורים, קרא שוב ל-API והעבר `parentId`.
   3. פתרון נפוץ הוא לבצע קריאת API לקבלת תגובות ברמה העליונה ולאחר מכן לבצע קריאות API מקבילות לקבלת תגובות הילדים של כל תגובה.
3. __NEW As of Feb 2023!__ ניתן לקבל כעץ באמצעות `&asTree=true`.
   1. ניתן לחשוב על זה כ-`Flexible Pagination as a Tree`.
   2. רק תגובות ברמה העליונה נספרות בדפדוף.
   3. הגדר `parentId=null` כדי להתחיל את העץ מהשורש (חובה להגדיר `parentId`).
   4. הגדר `skip` ו-`limit` לדפדוף.
   5. הגדר `asTree` ל-`true`.
   6. עלות הקרדיטים עולה ב-`2x`, מכיוון שהשרת שלנו צריך לבצע עבודה משמעותית יותר בתרחיש זה.
   7. הגדר את `maxTreeDepth`, `limitChildren` ו-`skipChildren` לפי הצורך.

### Trees Explained

כאשר משתמשים ב-`asTree`, קשה להבין את הדפדוף. הנה גרפיקה שימושית:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Fetching Comments in The Context of a User

ה-API `/comments` ניתן להשתמש בו בשני הקשרים, למקרים שונים:

- להחזרת תגובות ממוינות ומסומנות במידע לבניית הלקוח שלך.
  - במקרה זה, הגדר פרמטר שאילתה `contextUserId`.
- להשגת תגובות מהשרת שלך לשילובים מותאמים.
  - הפלטפורמה תשתמש בברירת מחדל זו ללא `contextUserId`.

[inline-code-attrs-start title = 'הערות דפדוף מחושב מראש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'הערות דפדוף גמיש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'הערות דפדוף גמיש בהקשר של משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'הערות דפדוף גמיש בהקשר של משתמש רק לתגובות ברמה העליונה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

אפשר לקבל את התגובות כמבנה עץ, כאשר הדפדוף סופר רק את התגובות ברמה העליונה.

[inline-code-attrs-start title = 'הערות כעץ בהקשר של משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

רוצים לקבל רק את התגובות ברמה העליונה ואת הילדים הישירים? הנה דרך אחת:

[inline-code-attrs-start title = 'הערות כעץ עם עומק מרבי'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

עם זאת, בממשק המשתמש שלך ייתכן שתצטרך לדעת האם להציג כפתור "הצגת תגובות" על כל תגובה. כאשר משיגים תגובות באמצעות עץ, קיימת תכונה `hasChildren` שמתוייגת לתגובות במידת הצורך.

### Get Comments as a Tree, Searching by Hash Tag

אפשר לחפש לפי תגית באמצעות ה-API, בכל השוכר שלך (לא מוגבל לעמוד אחד או ל-`urlId`).

בדוגמה זו, אנו משאירים את `urlId` ריק, ומחפשים לפי מספר תגיות. ה-API יחזיר רק תגובות שמכילות את כל התגיות המבוקשות.

[inline-code-attrs-start title = 'הערות כעץ בהקשר של משתמש, לפי תגית'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'מבנה בקשת הערות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ה-`urlId` (כתובת העמוד או מזהה המאמר) שהתגובות משויכות אליו. **/
    urlId?: string
    /** מגביל את התגובות המוחזרות על ידי משתמש זה. **/
    userId?: string
    /** השתמש בזה לחיפוש לפי תגית. כדי לצלול לחיתוך של מספר תגיות, השתמש ב-&hashTag=a&hashTag=b. **/
    hashTag?: string
    /** כיוון המיון. ברירת המחדל היא MR (הכי רלוונטי). אפשרויות אחרות הן OF (הישן ביותר ראשון) ו-NF (החדש ביותר ראשון). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** דפדוף מחושב מראש: העמוד שיש להביא, מתחיל ב-0. העבר -1 לכל התגובות (עד 250). **/
    page?: number
    /** דפדוף גמיש: כמה תגובות עלינו להחזיר? **/
    limit?: number
    /** דפדוף גמיש: כמה תגובות ילדים יש להחזיר לכל הורה? **/
    limitChildren?: number
    /** דפדוף גמיש: כמה תגובות יש לדלג? **/
    skip?: number
    /** דפדוף גמיש: כמה תגובות ילדים יש לדלג עבור כל הורה? **/
    skipChildren?: number
    /** לקביעת תגובות חסומות ומסומנות. **/
    contextUserId?: string
    /** לקביעת תגובות חסומות ומסומנות. **/
    anonUserId?: string
    /** לקבלת תגובות ילדים. **/
    parentId?: string
    /** לקבלת תגובות כעץ. **/
    asTree?: boolean
    /** עד כמה בעץ נחזיר נתונים? 0 מחזיר ללא ילדים. 1 מחזיר את הילדים הישירים, וכן הלאה. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'מבנה תגובת הערות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** נכלל במקרה של כשל. **/
    reason?: string
    /** התגובות! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

סביר להניח שתרצה להשתמש ב-API `Comment` עם הפרמטר `urlId`. ניתן לקרוא ל-API `Pages` תחילה, כדי לראות איך נראות ערכי `urlId` הזמינים עבורך.

#### Anonymous Actions

בצורת תגובה אנונימית סביר להניח שתרצה להעביר `anonUserId` בעת שליפת תגובות, וכן בעת סימון והחסימה.

(!) זה נדרש ברבים מחנויות האפליקציות מכיוון שמשתמשים חייבים להיות מסוגלים לסמן תוכן שנוצר על ידי משתמשים שהם רואים, גם אם הם לא מחוברים. חוסר פעולה זו עלול לגרום להסרת האפליקציה שלך מהחנות.

#### Comments Not Being Returned

ודא שהתגובות שלך אושרו ואינן ספאם.

---