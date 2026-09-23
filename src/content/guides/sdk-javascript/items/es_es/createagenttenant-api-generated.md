Crea una nueva cuenta de prueba para un agente de IA sin un registro humano. No se necesita una clave API para llamar a este.

La respuesta contiene el ID del inquilino, una clave API que funciona inmediatamente contra la API REST y el servidor MCP, y una URL de reclamación. Proporcione la URL de reclamación a la persona para la que está trabajando: abrirla mientras está conectado a FastComments asocia la cuenta a ella. Las cuentas no reclamadas, y sus claves, se eliminan 72 horas después de su creación. Hasta que se reclamen, la cuenta tiene los límites de prueba estándar.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Devuelve: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // opcional
  planId: 3 // opcional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]