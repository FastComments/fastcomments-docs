하나의 urlId 또는 도메인에서 다른 urlId 또는 도메인으로 댓글, 페이지 및 관련 레코드를 이동하는 배치 작업을 큐에 추가합니다.
"urlId" 마이그레이션은 정확히 일치하는 urlId만 대상으로 합니다. "domain" 마이그레이션은 모든 urlId와 url 내에서 값을 찾아 교체합니다.
GET /api/v1/batch-jobs/{batchJobId}를 폴링하세요: 완료되면 마이그레이션별 결과와 undoId를 포함합니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환값: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## 예제

[inline-code-attrs-start title = 'migratePages 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let migrateAPIPagesData = MigrateAPIPagesData(migrations: [PageMigration(from: "from_example", to: "to_example", type: PageMigrationType())]) // MigrateAPIPagesData | 

DefaultAPI.migratePages(tenantId: tenantId, migrateAPIPagesData: migrateAPIPagesData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]