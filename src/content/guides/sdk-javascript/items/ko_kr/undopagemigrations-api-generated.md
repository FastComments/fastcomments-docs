하나의 migrate-pages 작업에서 발생한 모든 마이그레이션을 최신 항목부터 역순으로 되돌리는 배치 작업을 큐에 추가합니다. 90일 동안 사용할 수 있습니다.
마이그레이션 이후 변경된 레코드는 그대로 유지되므로, 이후의 편집 내용이 덮어써지는 일은 없습니다.
복원됨/건너뜀 개수를 확인하려면 GET /api/v1/batch-jobs/{batchJobId}를 폴링하세요.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| undoId | string | 예 |  |

## 응답

반환값: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'undoPageMigrations 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]