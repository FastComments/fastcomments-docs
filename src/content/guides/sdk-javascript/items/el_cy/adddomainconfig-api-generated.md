## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| addDomainConfigParams | AddDomainConfigParams | Ναι |  |

## Απόκριση

Επιστρέφει: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-72";
  const addDomainConfigParams: AddDomainConfigParams = {
    domain: "comments.acme-corp.com",
    primary: true,
    enforceHttps: true,                // προαιρετική παράμετρος προς επίδειξη
    allowedOrigins: ["https://www.acme-corp.com", "https://app.acme-corp.com"],
    cnameTarget: "fc-cname.fastcomments.net"
  };
  const result: AddDomainConfigResponse = await addDomainConfig(tenantId, addDomainConfigParams);
  console.log(result);
})();
[inline-code-end]

---