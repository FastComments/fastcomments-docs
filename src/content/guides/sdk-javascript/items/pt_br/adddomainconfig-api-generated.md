## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| addDomainConfigParams | AddDomainConfigParams | Sim |  |

## Resposta

Retorna: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de addDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-72";
  const addDomainConfigParams: AddDomainConfigParams = {
    domain: "comments.acme-corp.com",
    primary: true,
    enforceHttps: true,                // parâmetro opcional demonstrado
    allowedOrigins: ["https://www.acme-corp.com", "https://app.acme-corp.com"],
    cnameTarget: "fc-cname.fastcomments.net"
  };
  const result: AddDomainConfigResponse = await addDomainConfig(tenantId, addDomainConfigParams);
  console.log(result);
})();
[inline-code-end]

---