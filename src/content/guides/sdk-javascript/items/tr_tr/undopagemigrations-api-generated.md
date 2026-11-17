Tek bir migrate-pages işindeki her geçişi, en yeniden başlayarak tersine çeviren bir toplu iş kuyruğa alır. 90 gün boyunca kullanılabilir.
Geçişten sonra değişen kayıtlara dokunulmaz, böylece sonraki bir düzenleme asla üzerine yazılmaz.
Geri yüklenen/atlanan sayımlar için GET /api/v1/batch-jobs/{batchJobId} uç noktasını yoklayın.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| undoId | string | Evet |  |

## Yanıt

Döndürür: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Örnek

[inline-code-attrs-start title = 'undoPageMigrations Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Geri alma kuyruğa alındı' : 'Geri alma başarısız oldu');
}

run();
[inline-code-end]