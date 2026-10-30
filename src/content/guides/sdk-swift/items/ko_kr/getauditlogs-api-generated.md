## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | 감사 로그를 읽을 테넌트. |
| limit | integer | query | No | 반환할 로그 수. 최대 10000, 기본값은 1000. |
| skip | integer | query | No | 페이지 매김을 위해 건너뛸 로그 수. |
| order | string | query | No | 이벤트 시간에 대한 정렬 방향. 기본값은 DESC (최신 순). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | 이 사용자 이름으로 수행된 이벤트만 반환. |
| ip | string | query | No | 이 IP 주소에서 발생한 이벤트만 반환. |
| crudType | string | query | No | 이 유형(c, r, u, d, login)의 이벤트만 반환. |
| resourceName | string | query | No | 이 리소스(예: User 또는 Moderator)의 이벤트만 반환. |
| targetId | string | query | No | 영향을 받은 객체가 이 ID를 가진 이벤트만 반환. |
| target | string | query | No | 영향을 받은 객체의 라벨(예: 사용자 이름 또는 이메일)에 대한 대소문자 구분 없는 부분 문자열 매치. |
| includeManagedTenants | boolean | query | No | 이 테넌트가 관리하는 테넌트의 이벤트도 반환. |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## 예시

[inline-code-attrs-start title = 'getAuditLogs 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 감사 로그를 읽을 테넌트.
let limit = 987 // Int | 반환할 로그 수. 최대 10000, 기본값은 1000. (옵션)
let skip = 987 // Int | 페이지 매김을 위해 건너뛸 로그 수. (옵션)
let order = SORT_DIR() // SORTDIR | 이벤트 시간에 대한 정렬 방향. 기본값은 DESC(최신 순). (옵션)
let after = 987 // Int64 |  (옵션)
let before = 987 // Int64 |  (옵션)
let username = "username_example" // String | 이 사용자 이름으로 수행된 이벤트만 반환. (옵션)
let ip = "ip_example" // String | 이 IP 주소에서 발생한 이벤트만 반환. (옵션)
let crudType = "crudType_example" // String | 이 유형(c, r, u, d, login)의 이벤트만 반환. (옵션)
let resourceName = "resourceName_example" // String | 이 리소스(예: User 또는 Moderator)의 이벤트만 반환. (옵션)
let targetId = "targetId_example" // String | 영향을 받은 객체가 이 ID를 가진 이벤트만 반환. (옵션)
let target = "target_example" // String | 영향을 받은 객체의 라벨(예: 사용자 이름 또는 이메일)에 대한 대소문자 구분 없는 부분 문자열 매치. (옵션)
let includeManagedTenants = true // Bool | 이 테넌트가 관리하는 테넌트의 이벤트도 반환. (옵션)

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

---