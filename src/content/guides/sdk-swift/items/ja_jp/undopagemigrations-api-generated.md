1つのmigrate-pagesジョブからのすべてのマイグレーションを新しいものから順に元に戻すバッチジョブをキューに入れます。90日間利用可能です。
マイグレーション以降に変更されたレコードはそのまま残されるため、後の編集が上書きされることはありません。
復元/スキップされた数を確認するには、GET /api/v1/batch-jobs/{batchJobId} をポーリングしてください。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| undoId | string | path | はい |  |

## レスポンス

戻り値: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## 例

[inline-code-attrs-start title = 'undoPageMigrations の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題が発生した場合は、http://github.com/OpenAPITools/openapi-generator/issues/new から報告してください
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