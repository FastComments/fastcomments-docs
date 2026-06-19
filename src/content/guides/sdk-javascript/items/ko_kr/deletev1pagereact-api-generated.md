## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## 예제

[inline-code-attrs-start title = 'deleteV1PageReact 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const urlId: string = 'post-5f2a8b3c9d';
const deletedReaction: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
[inline-code-end]

---