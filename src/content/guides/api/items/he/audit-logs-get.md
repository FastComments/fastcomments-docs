[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי הפרמטרים `skip`, `limit`, `before` ו-`after`. AuditLogs מוחזרים בעמודים של `5000` כברירת מחדל, עד למגבלה מרבית של `limit` של `10000`, ממוינים לפי `when` ו-`id`. העמודים גדולים מכיוון שהקצה זה משמש בדרך כלל לייצוא היסטוריה במקום לעבור עליו באופן אינטראקטיבי.

כל `100` לוגים שמוחזרים עולה `1` קרדיט.

בברירת מחדל, תקבל רשימה עם **הפריטים החדשים ביותר ראשונים**. כך תוכל לבצע סקר החל מ-`skip=0`, לעבור על העמודים עד שתמצא את הרשומה האחרונה שצברת.

לחלופין, ניתן למיין מהישן לחדש, ולעבור על העמודים עד שלא נותרו רשומות.

ניתן למיין על ידי הגדרת `order` ל-`ASC` או `DESC`. ברירת המחדל היא `DESC`.

ניתן לבצע שאילתות לפי תאריך באמצעות `before` ו-`after` כתזמונים במילישניות. `before` ו-`after` אינם כולליים, וכל אחד מהם ניתן לשימוש באופן עצמאי.

## מציאת מה שקרה לאדם

כל אירוע מתעד מי ביצע אותו (`username`, `userId`, `ip`) ובנפרד, על מה הוא בוצע. `targetLabel` הוא תווית קריאה לבן אדם עבור האובייקט, לדוגמה `jsmith (jsmith@example.com)`, ו-`targetId` הוא המזהה שלו. השתמש ב-`target` להתאמת תת‑מחרוזת ללא תלות ברישיות על התווית כאשר אתה יודע את שם האדם או האימייל אך לא את המזהה שלו.

מחיקות תופסות את התווית בזמן האירוע, כך שמשתמש או מודרטור שהוסרו עדיין ניתן לזהות לאחר שהרשומה הבסיסית נעלמה.

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
    /** Max 10000. Defaults to 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---