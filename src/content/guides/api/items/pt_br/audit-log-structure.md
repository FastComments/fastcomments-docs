Um `AuditLog` é um objeto que representa um evento auditado para tenants que têm acesso a este recurso.

A estrutura do objeto `AuditLog` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

O registro de auditoria é imutável. Também não pode ser escrito manualmente. Somente a FastComments.com pode decidir quando gravar no registro de auditoria. No entanto, você pode lê-lo através desta API.

Os eventos no registro de auditoria expiram após dois anos.