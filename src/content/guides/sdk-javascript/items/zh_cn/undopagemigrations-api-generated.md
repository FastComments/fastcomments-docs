将一个批处理任务加入队列，该任务会按从新到旧的顺序撤销某个 migrate-pages 任务的每一次迁移。此功能可用 90 天。
迁移之后发生变更的记录不会被改动，因此后续的编辑永远不会被覆盖。
轮询 GET /api/v1/batch-jobs/{batchJobId} 以获取已恢复/已跳过的数量。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| undoId | string | 是 |  |

## 响应

返回：[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'undoPageMigrations 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? '撤销已加入队列' : '撤销失败');
}

run();
[inline-code-end]