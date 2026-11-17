מוסיף משימת אצווה (batch job) שמבטלת כל העברה (migration) ממשימת migrate-pages אחת, החדשות ביותר תחילה. זמין למשך 90 יום.
רשומות שהשתנו מאז ההעברה נשארות ללא שינוי, כך שעריכה מאוחרת לעולם לא תידרס.
יש לבצע Polling ל-GET /api/v1/batch-jobs/{batchJobId} עבור מספרי השחזורים/הדילוגים.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| undoId | string | כן |  |

## תגובה

מחזיר: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]