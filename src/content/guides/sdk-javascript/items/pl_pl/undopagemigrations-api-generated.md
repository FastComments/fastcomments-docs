Kolejkuje zadanie wsadowe, które cofa każdą migrację z jednego zadania migrate-pages, od najnowszych. Dostępne przez 90 dni.
Rekordy zmienione od czasu migracji pozostają nietknięte, więc późniejsza edycja nigdy nie zostanie nadpisana.
Odpytuj GET /api/v1/batch-jobs/{batchJobId}, aby uzyskać liczby przywróconych/pominiętych.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| undoId | string | Tak |  |

## Odpowiedź

Zwraca: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Cofanie zakolejkowane' : 'Cofanie nie powiodło się');
}

run();
[inline-code-end]