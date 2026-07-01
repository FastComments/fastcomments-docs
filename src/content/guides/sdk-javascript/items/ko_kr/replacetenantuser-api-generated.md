## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 예 |  |
| updateComments | string | 아니오 |  |

## 응답

반환: [`ReplaceTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantUserResponse.ts)

## 예시

[inline-code-attrs-start title = 'replaceTenantUser 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function updateUser() {
  const tenantId: string = "c3d1f2e4-5b6a-4c7d-9f2e-1234567890ab";
  const userId: string = "u-654321";
  const replaceBody: ReplaceTenantUserBody = {
    email: "newuser@example.com",
    username: "newusername"
  };
  const response: ReplaceTenantUserResponse = await replaceTenantUser(
    tenantId,
    userId,
    replaceBody,
    "true"
  );
  console.log(response);
}
updateUser();
[inline-code-end]

---