Kører et batchjob, der tilbagefører hver migration fra ét migrate-pages-job, nyeste først. Tilgængelig i 90 dage.
Poster, der er ændret siden migrationen, efterlades urørte, så en senere redigering aldrig overskrives.
Poll GET /api/v1/batch-jobs/{batchJobId} for at se antallet af gendannede/sprunne-over.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| undoId | string | Ja |  |

## Svar

Returnerer: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'undoPageMigrations Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]