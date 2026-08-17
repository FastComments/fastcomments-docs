## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sure | string | 아니오 |  |

## 응답

반환: [`DeleteTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteTenant 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "tenant_12345";
    const id: string = "resource_98765";
    const sure: string = "confirm";

    // 선택적 'sure' 매개변수와 함께 호출
    const responseWithSure: DeleteTenantResponse = await deleteTenant(tenantId, id, sure);
    console.log(responseWithSure);

    // 선택적 'sure' 매개변수 없이 호출
    const responseWithoutSure: DeleteTenantResponse = await deleteTenant(tenantId, id);
    console.log(responseWithoutSure);
}

runExample();
[inline-code-end]