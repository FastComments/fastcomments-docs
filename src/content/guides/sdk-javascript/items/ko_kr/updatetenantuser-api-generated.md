## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantUserBody | UpdateTenantUserBody | 예 |  |
| updateComments | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예시

[inline-code-attrs-start title = 'updateTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_987";
  const updateBody: UpdateTenantUserBody = {
    email: "new.email@example.com",
    role: "admin",
    isActive: true
  };
  const comment: string = "Promoted to admin role";

  const result: APIEmptyResponse = await updateTenantUser(tenantId, userId, updateBody, comment);
  console.log(result);
}

runUpdate();
[inline-code-end]

---