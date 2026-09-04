Um `AuditLog` é um objeto que representa um evento auditado para locatários que têm acesso a este recurso.

A estrutura do objeto AuditLog é a seguinte:

[inline-code-attrs-start title = 'Estrutura do AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Quem realizou o evento. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** O navegador que realizou o evento, quando ele veio de um. **/
    ua?: string;
    /** Um hash da sessão de onde o evento veio, para correlacionar as ações de uma pessoa. Nunca a própria sessão. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** O id do objeto no qual o evento foi realizado, em oposição a quem o realizou. **/
    targetId?: string;
    /** Um rótulo legível para esse objeto, por exemplo "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` e `targetLabel` descrevem sobre o que o evento foi realizado; `userId` e `username` descrevem quem o realizou. Para atualizações, `objectDetails.changes` contém um mapa `{field: {from, to}}` do que realmente mudou.

O registro de auditoria é imutável. Também não pode ser escrito manualmente. A FastComments.com pode decidir quando escrever no registro de auditoria. No entanto, você pode lê-lo através desta API.

Os eventos no registro de auditoria expiram após dois anos.