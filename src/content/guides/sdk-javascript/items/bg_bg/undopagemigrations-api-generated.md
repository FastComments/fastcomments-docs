Подрежда пакетна задача, която обръща всяка миграция от една migrate-pages задача, започвайки от най-новата. Достъпно в продължение на 90 дни.
Записите, променени след миграцията, не се засягат, така че по-късна редакция никога не се презаписва.
Изпитвайте GET /api/v1/batch-jobs/{batchJobId} за броя на възстановените/пропуснатите записи.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| undoId | string | Да |  |

## Отговор

Връща: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]