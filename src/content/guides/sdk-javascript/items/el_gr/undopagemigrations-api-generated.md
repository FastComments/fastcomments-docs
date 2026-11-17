Σειροθέτησε μια εργασία δέσμης που αντιστρέφει κάθε μετανάστευση από μία εργασία migrate-pages, ξεκινώντας από τη νεότερη. Διαθέσιμη για 90 ημέρες.
Οι εγγραφές που άλλαξαν μετά τη μετανάστευση δεν επηρεάζονται, ώστε μια μεταγενέστερη επεξεργασία να μην ποτέ αντικαθίσταται.
Κάντε polling στο GET /api/v1/batch-jobs/{batchJobId} για τα πλήθη επαναφοράς/παράλειψης.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| undoId | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]