Accoda un job batch che inverte ogni migrazione di un job migrate-pages, dal più recente al più vecchio. Disponibile per 90 giorni.
I record modificati dopo la migrazione vengono lasciati invariati, quindi una modifica successiva non viene mai sovrascritta.
Interroga GET /api/v1/batch-jobs/{batchJobId} per i conteggi di ripristinati/saltati.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| undoId | string | Sì |  |

## Risposta

Restituisce: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]