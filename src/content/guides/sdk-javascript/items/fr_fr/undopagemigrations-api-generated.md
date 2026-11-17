Met en file d'attente un traitement par lots qui annule chaque migration d'un job migrate-pages, du plus récent au plus ancien. Disponible pendant 90 jours.
Les enregistrements modifiés depuis la migration ne sont pas touchés, une édition ultérieure n'est donc jamais écrasée.
Interrogez GET /api/v1/batch-jobs/{batchJobId} pour obtenir les compteurs restaurés/ignorés.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| undoId | string | Oui |  |

## Réponse

Retourne : [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]