## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| contextUserId | string | 아니오 |  |
| isLive | boolean | 아니오 |  |

## 응답

반환: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84a9f2';
const id: string = 'comment_5f3b21';
const contextUserId: string | undefined = 'user_1122';
const isLive: boolean | undefined = true;

async function run(): Promise<void> {
  const result: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
  console.log(result);
}

run();
[inline-code-end]

---