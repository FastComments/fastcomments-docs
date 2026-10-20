[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

נתיב זה מבטל כל הגירה מתוך משימת `POST /api/v1/pages/migrations`. קבל את ה-`undoId` מהמשימה שהסתיימה עם `GET /api/v1/batch-jobs/:id`. ניתן להשתמש ב-`undoId` במשך 90 יום, ופעם אחת בלבד.

הביטול רץ כמשימת אצווה, בדיוק כמו ההגירה, והתגובה מכילה את ה-`batchJobId` שלה. כאשר המשימה מסתיימת, `pageMigrationUndoResult` מדווח:

- `restored`: רשומות שהוחזרו לערכיהן לפני ההגירה.
- `skipped`: רשומות שנשארו ללא שינוי מכיוון שהשתנו לאחר ההגירה, או שאינן קיימות יותר. עריכה מאוחרת יותר לעולם לא תידרס.
- `notRestorable`: רשומות שההגירה מחקה במהלך מיזוג של שני דפים, או בעת הסרת תגובות דף משוכפל. לא ניתן לשחזר אותן.

הגירות מתבצעות הפוך מהחדשות ביותר, כך שרצף כמו `a` ל-`b` ולאחר מכן `b` ל-`c` מתפרק חזרה ל-`a`. משימת הגירה שנכשלה חלקית עדיין ניתנת לביטול, מה שמחזיר את השינויים שנעשו לפני הכשל.

הצבת ביטול בתור אינה עולה דבר. כאשר המשימה מסתיימת, היא עולה `1` קרדיט לכל `1000` רשומות שהוחזרו, עם מינימום של `1`.

[inline-code-attrs-start title = 'דוגמת cURL לביטול הגירות דפים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת ביטול הגירות דפים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת ביטול הגירות דפים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * כלול במקרה של כשל.
     * undo-not-found (404): אין הגירה עם undoId זה, או שהוא ישן יותר מ‑90 יום.
     * already-undone (409) ו‑undo-in-progress (409): ההגירה בוטלה, או שביטול שלה נמצא בתור או רץ.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** כלול במקרה של כשל. **/
    reason?: string
    /** העבר ל‑GET /api/v1/batch-jobs/:id לקבלת התקדמות והתוצאה. **/
    batchJobId?: string
}
[inline-code-end]