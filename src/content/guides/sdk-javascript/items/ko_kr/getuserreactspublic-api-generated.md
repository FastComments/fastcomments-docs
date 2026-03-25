## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | Array<string> | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserReactsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-8a4d2c";
const postIds: string[] = ["post-645a2f", "post-645a30"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyMyIsImlhdCI6MTY2MTYwMDAwMH0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const result: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
[inline-code-end]

---