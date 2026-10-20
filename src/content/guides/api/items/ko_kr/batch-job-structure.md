A `BatchJob`은 가져오기, 내보내기 또는 [페이지 마이그레이션](#pages-migrations-post)과 같은 장기 실행 작업입니다. 작업은 대기열에 들어가며, 이후 백그라운드에서 가져와 실행됩니다.

`processedCount`와 `totalCount`는 진행 상황을 표시합니다. 단위는 `type`에 따라 달라집니다:

- `migrate-pages`: 요청된 마이그레이션 중 완료된 마이그레이션 수.
- `undo-page-migration`: 복원된 레코드 수. `totalCount`는 없습니다.
- `import`, `export` 및 기타 작업: 처리된 행 수.

`BatchJob` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'BatchJob 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** 예: migrate-pages, undo-page-migration, migrate-domains, import, export, users-export 또는 copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** 사전에 알려진 경우 전체 작업량. **/
    totalCount?: number
    /** migrate-pages: 완료된 마이그레이션당 하나의 항목, 요청 순서대로. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: 작업이 완료된 후(성공 또는 실패) 설정됩니다. 90일 이내에 POST /api/v1/pages/migrations/:undoId/undo 로 전달하십시오.
     * undo-page-migration: 되돌리는 마이그레이션입니다.
     **/
    undoId?: string
    /** undo-page-migration: 작업이 완료된 후 설정됩니다. **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** 마이그레이션 이전 값으로 복원된 레코드. **/
    restored: number
    /** 마이그레이션 후 변경되었거나 더 이상 존재하지 않아 그대로 남은 레코드. **/
    skipped: number
    /** 페이지 병합 중 삭제된 마이그레이션 레코드. 이는 복구할 수 없습니다. **/
    notRestorable: number
}
[inline-code-end]

#### 재시도

페이지 마이그레이션 작업을 실행 중인 서버가 재시작하면, 작업은 대기열에 다시 들어가며 마지막으로 완료된 마이그레이션 이후부터 다시 진행됩니다. 작업은 최대 3번 시도되며, 그 이후에는 `status`가 `failed`가 됩니다.