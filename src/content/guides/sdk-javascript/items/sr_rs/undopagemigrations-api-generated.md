Креира пакетни посао који обрће сваку миграцију једног migrate-pages посла, од најновије. Доступно 90 дана.
Записи измењени од миграције остају нетакнути, тако да каснија измена никада неће бити преписана.
Питајте GET /api/v1/batch-jobs/{batchJobId} за бројеве враћених/прескочених ставки.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| undoId | string | Да |  |

## Одговор

Враћа: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'undoPageMigrations пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]