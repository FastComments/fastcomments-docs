## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`DeleteV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReact200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteV2PageReact 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_79021";
const urlId: string = "blog/my-first-post";
const id: string = "reaction_9f8b7c";
let includeHistory: boolean | undefined = undefined; // 선택적 플래그, 일부 호출에서 사용됨

const result: DeleteV2PageReact200Response = await deleteV2PageReact(tenantId, urlId, id);
console.log(result);
[inline-code-end]

---