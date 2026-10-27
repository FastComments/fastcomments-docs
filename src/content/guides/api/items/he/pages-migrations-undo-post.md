[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

נתיב זה מבטל כל מיגרציה ממטלה אחת של [`POST /api/v1/pages/migrations`](#pages-migrations-post). קבל את ה-`undoId` ממטלת המיגרציה שהסתיימה עם [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). ניתן להשתמש ב-`undoId` במשך 90 יום, ופעם אחת בלבד.

הביטול רץ כמטלת אצווה, בדיוק כמו המיגרציה, והתגובה מכילה את ה-`batchJobId` שלה. כאשר המטלה מסתיימת, `pageMigrationUndoResult` שלה מדווח:

- `restored`: רשומות שהוחזרו לערכיהן לפני המיגרציה.
- `skipped`: רשומות שנשארו ללא שינוי מכיוון שהשתנו לאחר המיגרציה, או שאינן קיימות יותר. עריכה מאוחרת יותר לעולם לא תידרס.
- `notRestorable`: רשומות שהמיגרציה מחקה במהלך מיזוג של שתי דפים, או בעת הסרת תגובות דף משוכפל. לא ניתן לשחזר אותן.

המיגרציות מתבצעות במצב הפוך, מהחדשה ביותר לישנה, ולכן שרשרת כמו `a ל‑b` ולאחר מכן `b ל‑c` מתפרקת חזרה ל‑`a`. מטלת מיגרציה שנכשלה בחלקה עדיין ניתנת לביטול, מה שמחזיר את השינויים שנעשו לפני הכשל.

הצבת ביטול בתור אינה עולה דבר. כאשר המטלה מסתיימת, היא עולה `1` קרדיט לכל `1000` רשומות שהוחזרו, עם מינימום של `1`.

[inline-code-attrs-start title = 'דוגמת cURL לביטול מיגרציות דף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת ביטול מיגרציות דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת ביטול מיגרציות דף'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * כלול במקרה של כשל.
     * undo-not-found (404): אין מיגרציה עם undoId זה, או שהוא ישן יותר מ‑90 יום.
     * already-undone (409) ו‑undo-in-progress (409): המיגרציה בוטלה, או שביטול שלה בתור או רץ.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** כלול במקרה של כשל. **/
    reason?: string
    /** העבר ל‑GET /api/v1/batch-jobs/:id לקבלת התקדמות והתוצאה. **/
    batchJobId?: string
}
[inline-code-end]