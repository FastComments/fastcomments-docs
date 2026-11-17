Створює пакетне завдання, яке скасовує кожну міграцію з одного завдання migrate-pages, починаючи з найновіших. Доступно протягом 90 днів.
Записи, змінені після міграції, залишаються без змін, тому пізніше редагування ніколи не перезаписується.
Опитуйте GET /api/v1/batch-jobs/{batchJobId}, щоб отримати кількість відновлених/пропущених записів.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| undoId | string | Так |  |

## Відповідь

Повертає: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Скасування поставлено в чергу' : 'Скасування не вдалося');
}

run();
[inline-code-end]