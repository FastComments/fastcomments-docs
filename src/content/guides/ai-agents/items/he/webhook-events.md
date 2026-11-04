ישנם ארבעה סוגי אירועי webhook של סוכן. לכל אירוע יש ערך enum מספרי (המשמש ב-payloads) ושם מחרוזת קנוני (המשמש בשדה ה-`event` של המעטפת ובכותרת ה-HTTP `X-FastComments-Agent-Event`).

| שם האירוע | Enum | מופעל כאשר |
|---|---|---|
| `trigger.succeeded` | 0 | ריצת סוכן מסתיימת עם סטטוס `SUCCESS`. |
| `trigger.failed` | 1 | ריצת סוכן מסתיימת עם סטטוס `ERROR`. |
| `approval.requested` | 2 | אישור נכנס לתור במצב `PENDING`. |
| `approval.decided` | 3 | אישור עובר ל-`APPROVED`, `REJECTED` או `EXECUTION_FAILED`. |

### `trigger.succeeded`

מופעל לאחר שריצת הסוכן מסתיימת ללא שגיאה. שדה ה-`data` של ה-payload כולל:

- `triggerId` - מזהה הריצה הייחודי.
- `triggerType` - ה-[enum של סיבת ההפעלה](#triggers-overview) שהתחיל את הריצה.
- `status` - `SUCCESS` (מחרוזת).
- `tokensUsed` - הטוקנים שנצרכו בריצה זו.
- `wasDryRun` - true אם הסוכן היה ב-[מצב dry-run](#dry-run-mode).
- `actions` - מערך של רשומות `TenantAgentAction` (ראו [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - אם להפעלה היו אלה.

אם הריצה לא ביצעה פעולות, המערך `actions` יהיה ריק - זו ריצה מוצלחת שבה "הסוכן החליט לא לעשות דבר", דבר שימושי לדעת.

### `trigger.failed`

מופעל כאשר ריצה נכשלת בשגיאה. צורת ה-payload זהה ל-`trigger.succeeded`, עם `status: 'ERROR'` ושדה נוסף `errorMessage` המתאר מה השתבש. שגיאות אפשריות כוללות כשלים בקריאות LLM, כשלים בהפעלת כלים, ואזלת תקציב באמצע ריצה.

`actions` עשוי עדיין להכיל רשומות עבור קריאות כלים שהושלמו לפני השגיאה.

### `approval.requested`

מופעל ברגע שאישור נכנס לתור במצב `PENDING`. ה-payload כולל:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - הארגומנטים של הכלי **מועברים כפי שהם** מקריאת ה-LLM. הצורה תלויה בכלי ואינה חוזה ציבורי יציב - הסכמה עשויה להשתנות ככל שנוספים כלים חדשים.
- `createdAt`.
- `justification`, `confidence` - אם הסוכן סיפק אותם.
- `contextSnapshot` - הקשר התגובה / העמוד שאליו מתייחס האישור.

שימושי להעברת אישורים ממתינים לערוץ chat ops: בוט Slack הרשום ל-`approval.requested` יכול לפרסם את הפעולה והנימוק בערוץ מודרציה לסקירה מהירה.

### `approval.decided`

מופעל כאשר אישור יוצא מ-`PENDING`. ה-payload כולל:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` או `EXECUTION_FAILED`.
- `decidedBy` - מזהה המשתמש של המנחה שקיבל את ההחלטה.
- `decidedAt` - מתי קיבל את ההחלטה.
- `executedAt` - אם APPROVED, מתי הפלטפורמה ביצעה את הפעולה המאושרת.
- `executionResult` - אם APPROVED, מחרוזת המתארת את תוצאת הביצוע.
- `contextSnapshot` - הקשר התגובה / העמוד.

אירוע זה מכסה את כל תוצאות ההחלטה:

- **אושר ובוצע בהצלחה** -> `status: APPROVED`, `executedAt` מוגדר, `executionResult` הוא הודעת ההצלחה.
- **אושר אך הביצוע נכשל** -> `status: EXECUTION_FAILED`, `executedAt` מוגדר, `executionResult` מתאר את הכשל.
- **נדחה** -> `status: REJECTED`, `executedAt` הוא null, `executionResult` הוא null.

### כותרת (Header)

כל משלוח כולל כותרת HTTP `X-FastComments-Agent-Event` עם שם המחרוזת הקנוני של האירוע (`trigger.succeeded` וכו'). שימושי אם נקודת הקצה שלכם היא URL יחיד המטפל במספר סוגי אירועים.

### ראו גם

- [Webhook Payloads](#webhook-payloads) לסכמות payload מלאות לכל אירוע.
- [Webhook Signing](#webhook-signing) לסכימת ה-HMAC.
- [Webhook Retries](#webhook-retries) לסמנטיקת המשלוח.