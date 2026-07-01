---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filter | string | No |  |
| searchFilters | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

Returns: [`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## 예시

[inline-code-attrs-start title = 'getCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const count: GetCountResponse = await getCount({
    textSearch: "order issue",
    byIPFromComment: "198.51.100.23",
    filter: "pending",
    demo: true,
    tenantId: "acme_corp",
    sso: "sso_abcdef123456"
  });
  console.log(count);
}
main();
[inline-code-end]

---