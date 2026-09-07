[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי הפרמטרים `skip`, `limit`, `before`, ו-`after`. AuditLogs מוחזרים בעמודים של `1000` כברירת מחדל, עד למקסימום `limit` של `10000`, ממוינים לפי `when` ו-`id`. העמודים גדולים מכיוון שהקצה זה משמש בדרך כלל לייצוא היסטוריה במקום לעבור בעמודים באופן אינטראקטיבי.

כל `100` רשומות שמוחזרות בעלות קרדיט של `1`.

בברירת מחדל, תקבל רשימה עם **הפריטים החדשים ביותר ראשונים**. כך, תוכל לבצע polling החל מ-`skip=0`, לעבור בעמודים עד שתמצא את הרשומה האחרונה שצורכת.

לחלופין, אתה יכול למיין מהישן לחדש, ולעבור בעמודים עד שאין עוד רשומות.

ניתן למיין על ידי הגדרת `order` ל-`ASC` או `DESC`. ברירת המחדל היא `DESC`.

ניתן לבצע שאילתא לפי תאריך באמצעות `before` ו-`after` כתזמונים במילישניות. `before` ו-`after` אינם כולליים, וכל אחד מהם ניתן לשימוש בנפרד.

## מציאת מה קרה לאדם

כל אירוע מתעד מי ביצע אותו (`username`, `userId`, `ip`) ובנפרד, על מה הוא בוצע. `targetLabel` הוא תווית קריאה לבן אדם עבור האובייקט, לדוגמה `jsmith (jsmith@example.com)`, ו-`targetId` הוא המזהה שלו. השתמש ב-`target` להתאמת תת-מחרוזת ללא תלות ברישיות על התווית כאשר אתה יודע את שם האדם או האימייל אך לא את המזהה שלו.

מחיקות קולטות את התווית בזמן האירוע, ולכן משתמש או מודרטור שהוסרו עדיין ניתן לזהות לאחר שהרשומה הבסיסית נעלמה.

## שכירים מנוהלים

אם השוכר שלך מנהל שכירים אחרים, הגדר `includeManagedTenants=true` כדי לקבל אירועים מהשוכר שלך ומכל שוכר שהוא מנהל בתגובה אחת. ה-`tenantId` של כל לוג מוחזר מצביע על השוכר שממנו הוא הגיע.

[inline-code-attrs-start title = 'דוגמת cURL של AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** מקסימום 10000. ברירת מחדל 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** רק אירועים שבוצעו על ידי שם משתמש זה. **/
    username?: string
    /** רק אירועים מכתובת IP זו. **/
    ip?: string
    /** רק אירועים מסוג זה. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** רק אירועים עבור משאב זה, למשל משתמש או מודרטור. **/
    resourceName?: string
    /** רק אירועים שהאובייקט המושפע שלהם בעל מזהה זה. **/
    targetId?: string
    /** התאמת תת-מחרוזת ללא תלות ברישיות על תווית האובייקט המושפע. **/
    target?: string
    /** גם החזר אירועים משוכרים שהשוכר הזה מנהל. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** כלול בכשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** כלול בכשל. **/
    reason?: string
    /** הלוגים! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---