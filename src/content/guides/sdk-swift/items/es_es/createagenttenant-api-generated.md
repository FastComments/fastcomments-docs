Crea una nueva cuenta de prueba para un agente de IA sin un registro humano. No se necesita clave de API para llamar a esto.

La respuesta contiene el id del tenant, una clave de API que funciona de inmediato contra la API REST y el servidor MCP, y una URL de reclamación. Entrega la URL de reclamación a la persona para la que trabajas: al abrirla con la sesión iniciada en FastComments, la cuenta queda vinculada a esa persona. Las cuentas no reclamadas, junto con sus claves, se eliminan 72 horas después de su creación. Hasta que se reclame, la cuenta tiene los límites estándar de prueba.

## Respuesta

Devuelve: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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