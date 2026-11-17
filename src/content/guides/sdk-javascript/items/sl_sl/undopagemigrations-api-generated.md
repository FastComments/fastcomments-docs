Uvrsti paketno opravilo, ki obrne vsako migracijo iz enega opravila migrate-pages, od najnovejše naprej. Na voljo 90 dni.
Zapisi, spremenjeni od migracije, ostanejo nedotaknjeni, tako da kasnejše urejanje ni nikoli prepisano.
Anketiraj GET /api/v1/batch-jobs/{batchJobId} za število obnovljenih/izpuščenih.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| undoId | string | Da |  |

## Odgovor

Vrne: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]