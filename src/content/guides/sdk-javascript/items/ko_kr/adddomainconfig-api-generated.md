## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| addDomainConfigParams | AddDomainConfigParams | 예 |  |

## 응답

반환: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## 예제

[inline-code-attrs-start title = 'addDomainConfig 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-72";
  const addDomainConfigParams: AddDomainConfigParams = {
    domain: "comments.acme-corp.com",
    primary: true,
    enforceHttps: true,                // 선택적 매개변수 예시
    allowedOrigins: ["https://www.acme-corp.com", "https://app.acme-corp.com"],
    cnameTarget: "fc-cname.fastcomments.net"
  };
  const result: AddDomainConfigResponse = await addDomainConfig(tenantId, addDomainConfigParams);
  console.log(result);
})();
[inline-code-end]

---