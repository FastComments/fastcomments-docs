Un `AuditLog` es un objeto que representa un evento auditado para inquilinos que tienen acceso a esta función.

La estructura para el objeto AuditLog es la siguiente:

[inline-code-attrs-start title = 'Estructura de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    objectDetails?: object;
}
[inline-code-end]

El registro de auditoría es inmutable. Tampoco se puede escribir manualmente. FastComments.com es el único que puede decidir cuándo escribir en el registro de auditoría. Sin embargo, puede leer de él a través de esta API.

Los eventos en el registro de auditoría expiran después de dos años.
