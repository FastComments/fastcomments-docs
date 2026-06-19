테넌트의 대량 사용자 정보. userIds를 받아 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트로 막 등장한 사용자들을 보강하기 위해 사용됩니다.
페이지 컨텍스트 없음: 개인 정보 보호는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| ids | string | 예 |  |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo는 tenantId와 ids만 필요합니다; 선택적 매개변수는 여기서 적용되지 않습니다.
[inline-code-end]

---