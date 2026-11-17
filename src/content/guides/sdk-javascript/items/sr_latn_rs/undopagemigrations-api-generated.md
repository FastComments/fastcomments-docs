Postavlja batch posao koji poništava svaku migraciju iz jednog migrate-pages posla, od najnovije ka najstarijoj. Dostupno 90 dana.
Zapisi promenjeni od migracije se ostavljaju netaknutim, tako da kasnija izmena nikada neće biti prepisana.
Osluškujte GET /api/v1/batch-jobs/{batchJobId} za brojače vraćenih/preskočenih.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| undoId | string | Da |  |

## Odgovor

Vraća: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'undoPageMigrations primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]