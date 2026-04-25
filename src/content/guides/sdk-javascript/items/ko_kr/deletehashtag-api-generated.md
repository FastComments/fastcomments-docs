## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tag | string | 예 |  |
| tenantId | string | 아니오 |  |
| deleteHashTagRequest | DeleteHashTagRequest | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "spring-sale-2026";
const tenantId: string = "tenant-9876";
const deleteHashTagRequest: DeleteHashTagRequest = {
  requestedBy: "admin@retailco.com",
  reason: "Campaign ended; remove associated auto-tags",
  cascadeDelete: true
};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

---