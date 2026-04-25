## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| deleteComments | string | 아니오 |  |
| commentDeleteMode | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme_corp_tenant_9f1a2b";
  const id: string = "user_4d2a1b6c";
  const deleteComments: string = "true"; // 사용자의 댓글도 함께 제거
  const commentDeleteMode: string = "permanent"; // "permanent" 또는 "soft"
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
}
run();
[inline-code-end]

---