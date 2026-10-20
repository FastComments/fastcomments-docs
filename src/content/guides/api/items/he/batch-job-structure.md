A `BatchJob` הוא משימה ארוכת זמן, כמו ייבוא, ייצוא, או [הגירת דף](#pages-migrations-post). עבודות מתווספות לתור, ואז נלקחות ומורצות ברקע.

`processedCount` ו-`totalCount` מציגים התקדמות. היחידה שלהם תלויה ב-`type`:

- `migrate-pages`: הגירות שהושלמו, מתוך ההגירות שבקשה.
- `undo-page-migration`: רשומות ששוחזרו. אין `totalCount`.
- `import`, `export` ו‑עבודות אחרות: שורות שעובדו.

המבנה של האובייקט `BatchJob` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** לדוגמה migrate-pages, undo-page-migration, migrate-domains, import, export, users-export או copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** העבודה הכוללת, כאשר היא ידועה מראש. **/
    totalCount?: number
    /** migrate-pages: ערך אחד לכל הגירה שהושלמה, לפי סדר הבקשה. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: מוגדר לאחר שהעבודה הסתיימה (בהצלחה או בכישלון). שלח ל-POST /api/v1/pages/migrations/:undoId/undo בתוך 90 יום.
     * undo-page-migration: ההגירה שמבוטלת.
     **/
    undoId?: string
    /** undo-page-migration: מוגדר לאחר שהעבודה הסתיימה. **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** רשומות שהוחזרו לערכיהן לפני ההגירה. **/
    restored: number
    /** רשומות שנשארו ללא שינוי מכיוון שהשתנו לאחר ההגירה, או שאינן קיימות יותר. **/
    skipped: number
    /** רשומות שההגירה נמחקה בזמן מיזוג דפים. לא ניתן לשחזר אותן. **/
    notRestorable: number
}
[inline-code-end]

#### ניסיונות חוזרים

אם השרת שמריץ משימת הגירת דף מתאפס, המשימה מוחזרת לתור וממשיכה אחרי ההגירה האחרונה שהושלמה. משימה מנסה עד 3 פעמים, ולאחר מכן ה-`status` שלה הוא `failed`.