[api-resource-header-start name = 'QuestionConfig'; route = 'GET /api/v1/question-configs'; creditsCost = 1; api-resource-header-end]

Esta ruta devuelve hasta 100 objetos `QuestionConfig` a la vez, paginados. El costo es 1 por cada 100 objetos. Están
ordenados por texto de pregunta ascendente (campo `question`).

[inline-code-attrs-start title = 'Ejemplo de QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigRequestByIdQueryParams {
    tenantId: string
    API_KEY: string
    /** For pagination. Starts at 0. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigByIdResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    questionConfigs: QuestionConfig[]
}
[inline-code-end]
