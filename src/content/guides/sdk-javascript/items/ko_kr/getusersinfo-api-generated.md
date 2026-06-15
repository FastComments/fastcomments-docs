---
테넌트에 대한 대량 사용자 정보입니다. userIds가 주어지면 User / SSOUser에서 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트를 통해 방금 나타난 사용자들을 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호가 일관되게 적용됩니다(비공개 프로필은 가려집니다).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

반환: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Example

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // 선택 사항; undefined이면 기본값은 쉼표
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---