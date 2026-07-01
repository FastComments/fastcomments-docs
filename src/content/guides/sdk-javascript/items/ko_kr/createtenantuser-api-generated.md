## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantUserBody | CreateTenantUserBody | 예 |  |

## 응답

반환: [`CreateTenantUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse1.ts)

## 예시

[inline-code-attrs-start title = 'createTenantUser 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "a1b2c3d4-5678-90ab-cdef-1234567890ab";

const newUser: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "admin",
  // 옵션 매개변수
  phoneNumber: "+15551234567",
  isActive: true,
};

const result: CreateTenantUserResponse1 = await createTenantUser(tenantId, newUser);
[inline-code-end]