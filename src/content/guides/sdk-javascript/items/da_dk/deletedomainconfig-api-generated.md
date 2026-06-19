## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Ja |  |

## Svar

Returnerer: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteDomainConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_5b2f9c3a";
const domain: string = "comments.acme-corp.com";
const result: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
[inline-code-end]

---