Cria uma nova conta de teste para um agente de IA sem necessidade de cadastro humano. Não é necessária uma chave de API para chamar isso.

A resposta contém o ID do locatário, uma chave de API que funciona imediatamente contra a API REST e o
servidor MCP, e uma URL de reivindicação. Forneça a URL de reivindicação ao humano para quem você está trabalhando: ao abri‑la enquanto
estiver conectado ao FastComments, a conta será vinculada a ele. Contas não reivindicadas, e suas chaves, são
excluídas 72 horas após a criação. Até serem reivindicadas, a conta tem os limites padrão de teste.

## Resposta

Retorna: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O seguinte exemplo de código ainda está em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]