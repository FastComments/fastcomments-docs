배치 작업을 큐에 넣어 하나의 migrate-pages 작업에서 발생한 모든 마이그레이션을 최신 순으로 역전시킵니다. 작업은 90일 동안 사용할 수 있습니다.  
마이그레이션 이후 변경된 레코드는 그대로 유지되므로, 이후 편집이 덮어쓰여지지 않습니다.  
복원/건너뛴 수를 확인하려면 GET /api/v1/batch-jobs/{batchJobId}를 폴링하세요.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| undoId | string | path | Yes |  |

## 응답

반환: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## 예시

[inline-code-attrs-start title = 'undoPageMigrations 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
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