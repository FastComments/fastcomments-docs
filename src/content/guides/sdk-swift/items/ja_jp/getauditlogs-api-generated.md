## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | 監査ログを取得するテナント。 |
| limit | integer | query | No | 返却するログの数。最大 10000、デフォルトは 1000。 |
| skip | integer | query | No | ページングのためにスキップするログ数。 |
| order | string | query | No | イベント時間のソート方向。デフォルトは DESC（新しい順）。 |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | このユーザー名で実行されたイベントのみ返却。 |
| ip | string | query | No | この IP アドレスからのイベントのみ返却。 |
| crudType | string | query | No | このタイプ（c、r、u、d、または login）のイベントのみ返却。 |
| resourceName | string | query | No | このリソース（例: User または Moderator）のイベントのみ返却。 |
| targetId | string | query | No | 対象オブジェクトがこの ID を持つイベントのみ返却。 |
| target | string | query | No | 対象オブジェクトのラベルに対する大文字小文字を区別しない部分文字列一致、例: ユーザー名やメールアドレス。 |
| includeManagedTenants | boolean | query | No | このテナントが管理するテナントのイベントも返却。 |

## 応答

返却: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## 例

[inline-code-attrs-start title = 'getAuditLogs 例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 監査ログを取得するテナント。
let limit = 987 // Int | 返却するログの数。最大 10000、デフォルトは 1000。（オプション）
let skip = 987 // Int | ページングのためにスキップするログ数。（オプション）
let order = SORT_DIR() // SORTDIR | イベント時間のソート方向。デフォルトは DESC（新しい順）。（オプション）
let after = 987 // Int64 |  （オプション）
let before = 987 // Int64 |  （オプション）
let username = "username_example" // String | このユーザー名で実行されたイベントのみ返却。（オプション）
let ip = "ip_example" // String | この IP アドレスからのイベントのみ返却。（オプション）
let crudType = "crudType_example" // String | このタイプ（c、r、u、d、または login）のイベントのみ返却。（オプション）
let resourceName = "resourceName_example" // String | このリソース（例: User または Moderator）のイベントのみ返却。（オプション）
let targetId = "targetId_example" // String | 対象オブジェクトがこの ID を持つイベントのみ返却。（オプション）
let target = "target_example" // String | 対象オブジェクトのラベルに対する大文字小文字を区別しない部分文字列一致、例: ユーザー名やメールアドレス。（オプション）
let includeManagedTenants = true // Bool | このテナントが管理するテナントのイベントも返却。（オプション）

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]