---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateAPISSOUserData | UpdateAPISSOUserData | 예 |  |
| updateComments | boolean | 아니오 |  |

## 응답

반환: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'putSSOUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const id: string = 'usr-73a1b2';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'marcus.ingram@acme.com',
  givenName: 'Marcus',
  familyName: 'Ingram',
  roles: ['editor', 'project_owner'],
  enabled: true
};
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateAPISSOUserData, true);
[inline-code-end]

---