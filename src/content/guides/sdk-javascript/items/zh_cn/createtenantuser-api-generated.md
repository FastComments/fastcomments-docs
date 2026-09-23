## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 是 |  |

## 响应

返回: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## 示例

[inline-code-attrs-start title = 'createTenantUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function addTenantUser() {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const digestFreq: DigestEmailFrequency = 'daily';
  const body: CreateTenantUserBody = {
    email: 'jane.doe@example.com',
    firstName: 'Jane',
    lastName: 'Doe',
    role: 'admin',
    digestEmailFrequency: digestFreq,
    phoneNumber: '+15551234567' // 可选
  };
  const response: CreateTenantUserResponse = await createTenantUser(tenantId, body);
  console.log(response);
}
addTenantUser();
[inline-code-end]