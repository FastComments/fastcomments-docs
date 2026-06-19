## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| domain | string | Sì |  |

## Risposta

Restituisce: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_5b2f9c3a";
const domain: string = "comments.acme-corp.com";
const result: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
[inline-code-end]

---