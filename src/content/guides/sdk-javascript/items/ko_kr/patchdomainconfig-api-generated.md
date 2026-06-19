## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 예 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 예 |  |

## 응답

반환: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## 예제

[inline-code-attrs-start title = 'patchDomainConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // 선택적 매개변수 포함
  allowedOrigins: ["https://newsroom.example.com"] // 선택적 매개변수 포함
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]

---