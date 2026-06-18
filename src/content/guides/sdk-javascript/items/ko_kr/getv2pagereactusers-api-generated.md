## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetV2PageReactUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsers200Response.ts)

## 예제

[inline-code-attrs-start title = 'getV2PageReactUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "7421";
const urlId: string = "sports/london-marathon";
const id: string = "reactUser-3fa85f64-5717-4562-b3fc-2c963f66afa6";
const includeDeleted: boolean | undefined = undefined; // 선택적 플래그 (예시)

const result: GetV2PageReactUsers200Response = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---