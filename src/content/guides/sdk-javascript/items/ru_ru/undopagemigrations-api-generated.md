Ставит в очередь пакетное задание, которое отменяет все миграции одного задания migrate-pages, начиная с самых новых. Доступно в течение 90 дней.
Записи, изменённые после миграции, не затрагиваются, поэтому последующее редактирование никогда не будет перезаписано.
Опрашивайте GET /api/v1/batch-jobs/{batchJobId} для получения счётчиков восстановленных/пропущенных записей.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| undoId | string | Да |  |

## Ответ

Возвращает: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Отмена поставлена в очередь' : 'Отмена не удалась');
}

run();
[inline-code-end]