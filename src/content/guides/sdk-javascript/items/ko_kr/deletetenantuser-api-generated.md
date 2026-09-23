## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteTenantUser 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "u-112233";

  const result1: APIEmptyResponse = await deleteTenantUser(tenantId, userId);
  const result2: APIEmptyResponse = await deleteTenantUser(tenantId, userId, "true", "hard");

  console.log(result1, result2);
})();
[inline-code-end]

---