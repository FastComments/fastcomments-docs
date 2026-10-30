Queues a batch job that reverses every migration from one migrate-pages job, newest first. Available for 90 days.  
マイグレーションページジョブを逆順（最新から）にすべて元に戻すバッチジョブをキューに入れます。利用可能期間は90日です。

Records changed since the migration are left alone, so a later edit is never overwritten.  
マイグレーション以降に変更されたレコードはそのまま残されるため、後からの編集が上書きされることはありません。

Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.  
復元/スキップされた件数を取得するには GET /api/v1/batch-jobs/{batchJobId} をポーリングしてください。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| undoId | string | path | はい |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]