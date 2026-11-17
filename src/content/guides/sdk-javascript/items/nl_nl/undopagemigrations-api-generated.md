Zet een batchtaak in de wachtrij die elke migratie van één migrate-pages-taak ongedaan maakt, nieuwste eerst. 90 dagen beschikbaar.
Records die sinds de migratie zijn gewijzigd worden met rust gelaten, zodat een latere bewerking nooit wordt overschreven.
Poll GET /api/v1/batch-jobs/{batchJobId} voor de herstelde/overgeslagen aantallen.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| undoId | string | Ja |  |

## Response

Retourneert: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'undoPageMigrations Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]