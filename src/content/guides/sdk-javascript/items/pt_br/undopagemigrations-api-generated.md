Enfileira um job em lote que reverte todas as migrações de um job migrate-pages, do mais recente para o mais antigo. Disponível por 90 dias.
Os registros alterados desde a migração são deixados intactos, portanto uma edição posterior nunca é sobrescrita.
Consulte GET /api/v1/batch-jobs/{batchJobId} para obter as contagens de restaurados/ignorados.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| undoId | string | Sim |  |

## Resposta

Retorna: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]