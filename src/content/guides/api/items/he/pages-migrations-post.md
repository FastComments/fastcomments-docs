[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

הנתיב הזה מעביר תגובות ודפים מ-`urlId` או דומיין אחד לאחר, בדיוק כמו [כלי העברת הדומיינים](/guide-migrations.html#migrating-domains) בלוח הבקרה. ניתן לשלוח עד 100 העברות בכל בקשה. הן רצות לפי הסדר שניתן, כך שהעברה מ-`a` ל-`b` ואחריה מ-`b` ל-`c` עובדת.

לכל העברה יש `type`:

- `urlId` מעביר את הדף שה-`urlId` שלו זהה בדיוק ל-`from`. `post-1` אינו תואם ל-`post-10`.
- `domain` מבצע חיפוש והחלפה של `from` בתוך כל `urlId` ו-`url`, ומעדכן את ה-`domain` של כל תגובה. שני הערכים חייבים להיות דומיין, כמו `old.example.com`.

תגובות, הדף, תגובות ריאקציה לדף, הצבעות, התראות ומינויים – הכול עובר. אם כבר קיים דף ב-`to`, שני הדפים ממוזגים.

ההעברה רצה כעבודת אצווה. התשובה מכילה `batchJobId`. בצעו polling ל-[`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) כדי לעקוב אחר ההתקדמות. לאחר שהעבודה מסתיימת, יש לה תוצאה עבור כל העברה, וכן `undoId` שניתן להעביר ל-[`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) בתוך 90 יום.

הבקשה מאומתת לפני שמשהו נכנס לתור. אם העברה כלשהי אינה תקפה, כל הבקשה נכשלת עם `422` ושום דבר לא רץ.

הכנסת העברה לתור לא עולה דבר. כשהעבודה מסתיימת, העלות היא `1` קרדיט לכל `1000` תגובות שהועברו, עם מינימום של `1`.

[inline-code-attrs-start title = 'דוגמת cURL להעברת דפים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשה להעברת דפים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תשובה להעברת דפים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### הערות

- עבודות של טננט אחד רצות אחת בכל פעם, מהישנה לחדשה. העברה שנייה ממתינה לסיום הראשונה.
- `PATCH /api/v1/pages/:id` עם `urlId` חדש משנה רק את הדף, לא את התגובות שלו. השתמשו בנתיב הזה כדי להעביר את התגובות של דף.