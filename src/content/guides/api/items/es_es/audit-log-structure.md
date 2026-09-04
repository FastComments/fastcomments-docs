Un `AuditLog` es un objeto que representa un evento auditado para los inquilinos que tienen acceso a esta función.

La estructura del objeto AuditLog es la siguiente:

[inline-code-attrs-start title = 'Estructura de AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Quién realizó el evento. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** El navegador que realizó el evento, cuando provino de uno. **/
    ua?: string;
    /** Un hash de la sesión de la que proviene el evento, para correlacionar las acciones de una persona. Nunca la sesión completa. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** El id del objeto sobre el que se realizó el evento, en contraste con quién lo realizó. **/
    targetId?: string;
    /** Una etiqueta legible para ese objeto, por ejemplo "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` y `targetLabel` describen sobre qué se realizó el evento; `userId` y `username` describen quién lo realizó. Para actualizaciones, `objectDetails.changes` contiene un mapa `{field: {from, to}}` de lo que realmente cambió.

El registro de auditoría es inmutable. Además, no puede ser escrito manualmente. FastComments.com solo puede decidir cuándo escribir en el registro de auditoría. Sin embargo, puedes leerlo a través de esta API.

Los eventos en el registro de auditoría expiran después de dos años.