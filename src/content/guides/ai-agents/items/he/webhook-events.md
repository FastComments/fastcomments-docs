יש ארבעה סוגי אירועי webhook של סוכן. לכל אירוע יש ערך enum מספרי (שמשמש במטענים) ושם מחרוזת קנוני (המשמש בשדה המעטפה `event` ובכותרת ה-HTTP `X-FastComments-Agent-Event`).

| שם האירוע | Enum | מתרחש כאשר |
|---|---|---|
| `trigger.succeeded` | 0 | ריצת סוכן מסתיימת בסטטוס `SUCCESS`. |
| `trigger.failed` | 1 | ריצת סוכן מסתיימת בסטטוס `ERROR`. |
| `approval.requested` | 2 | בקשת אישור מתווספת לתור במצב `PENDING`. |
| `approval.decided` | 3 | אישור עובר ל-`APPROVED`, `REJECTED`, או `EXECUTION_FAILED`. |

### `trigger.succeeded`

נשלח לאחר שריצת הסוכן מסתיימת ללא שגיאות. שדה `data` במטען כולל:

- `triggerId` - מזהה הרצה ייחודי.
- `triggerType` - ה-[trigger reason enum](#triggers-overview) שהפעיל את הריצה.
- `status` - `SUCCESS` (מחרוזת).
- `tokensUsed` - מספר הטוקנים שנצרכו בהרצה זו.
- `wasDryRun` - true אם הסוכן היה במצב [dry-run mode](#dry-run-mode).
- `actions` - מערך של רשומות `TenantAgentAction` (ראה [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - אם הטריגר הכיל אותם.

אם הריצה לא ביצעה פעולות, מערך `actions` יהיה ריק - זו ריצה מוצלחת של "הסוכן החליט לא לעשות כלום", וזה שימושי לדעת.

### `trigger.failed`

נשלח כאשר ריצה נכשלה עם שגיאה. צורת המטען זהה ל-`trigger.succeeded`, עם `status: 'ERROR'` ושדה נוסף `errorMessage` המתאר מה השתבש. שגיאות אפשריות כוללות כשלי קריאות LLM, כשלי שליחת פקודות לכלים, וגמר תקציב באמצע הריצה.

למרות זאת, `actions` עדיין עשוי להכיל רשומות של קריאות לכלי שהושלמו לפני השגיאה.

### `approval.requested`

נשלח ברגע שבקשת אישור מתווספת לתור במצב `PENDING`. המטען כולל:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - ארגומנטים של הכלי **מועברים כפי שהם** מבקשת ה-LLM. המבנה הוא תלוי-כלי ואינו חוזה ציבורי יציב - ההסכמה יכולה להשתנות כאשר נוספים כלים חדשים.
- `createdAt`.
- `justification`, `confidence` - אם הסוכן סיפק אותם.
- `contextSnapshot` - הקשר ההערה/העמוד שקשור לאישור.

שימושי להעברה של אישורים ממתינים לערוץ chat ops: בוט Slack המנוי על `approval.requested` יכול לפרסם את הפעולה והנימוקים בערוץ מודרציה לצפייה מהירה.

### `approval.decided`

נשלח כשהאישור יוצא ממצב `PENDING`. המטען כולל:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, או `EXECUTION_FAILED`.
- `decidedBy` - מזהה המשתמש של המודרטור שהחליט.
- `decidedAt` - מתי החליט.
- `executedAt` - אם `APPROVED`, מתי הפלטפורמה ביצעה את הפעולה המאושרת.
- `executionResult` - אם `APPROVED`, מחרוזת המתארת את תוצאת הביצוע.
- `contextSnapshot` - הקשר ההערה/העמוד.

אירוע זה מכסה את כל תוצאות ההחלטה:

- **מאושר + בוצע בהצלחה** -> `status: APPROVED`, `executedAt` מוגדר, `executionResult` היא הודעת ההצלחה.
- **מאושר + הביצוע נכשל** -> `status: EXECUTION_FAILED`, `executedAt` מוגדר, `executionResult` מתאר את הכשל.
- **נדחה** -> `status: REJECTED`, `executedAt` הוא null, `executionResult` הוא null.

### Header

כל משלוח כולל כותרת HTTP `X-FastComments-Agent-Event` עם שם המחרוזת הקנוני של האירוע (`trigger.succeeded`, וכו'). זה שימושי אם נקודת הקצה שלך היא כתובת URL יחידה המטפלת במספר סוגי אירועים.

### See also

- [Webhook Payloads](#webhook-payloads) עבור סכמות המטען המלאות לכל אירוע.
- [Webhook Signing](#webhook-signing) עבור סכמת HMAC.
- [Webhook Retries](#webhook-retries) לגבי סמנטיקת המסירה.