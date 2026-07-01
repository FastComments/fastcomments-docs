## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |

## 응답

반환값: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getDomainConfigs 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchDomainConfigs(): Promise<void> {
  const tenantId: string = "acme-corp-567";
  const configs: GetDomainConfigsResponse = await getDomainConfigs(tenantId);
  console.log(configs);
}
[inline-code-end]

---