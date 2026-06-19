## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| addDomainConfigParams | AddDomainConfigParams | Ja |  |

## Svar

Returnerer: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'addDomainConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-72";
  const addDomainConfigParams: AddDomainConfigParams = {
    domain: "comments.acme-corp.com",
    primary: true,
    enforceHttps: true,                // valgfri parameter demonstreret
    allowedOrigins: ["https://www.acme-corp.com", "https://app.acme-corp.com"],
    cnameTarget: "fc-cname.fastcomments.net"
  };
  const result: AddDomainConfigResponse = await addDomainConfig(tenantId, addDomainConfigParams);
  console.log(result);
})();
[inline-code-end]

---