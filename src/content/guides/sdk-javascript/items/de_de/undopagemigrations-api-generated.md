Stellt einen Batch-Job in die Warteschlange, der jede Migration eines migrate-pages-Jobs rückgängig macht, beginnend mit der neuesten. 90 Tage lang verfügbar.
Seit der Migration geänderte Datensätze bleiben unangetastet, sodass eine spätere Bearbeitung nie überschrieben wird.
Fragen Sie GET /api/v1/batch-jobs/{batchJobId} ab, um die Anzahl der wiederhergestellten/übersprungenen Einträge zu erhalten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| undoId | string | Ja |  |

## Antwort

Gibt zurück: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'undoPageMigrations Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]