## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`DeleteV1PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV1PageReactResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteV1PageReact 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const urlId: string = "article-2024-06-01";

const response: DeleteV1PageReactResponse = await deleteV1PageReact(tenantId, urlId);
console.log(response);
[inline-code-end]

---