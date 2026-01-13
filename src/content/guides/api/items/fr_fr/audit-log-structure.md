Un `AuditLog` est un objet qui représente un événement audité pour les locataires qui ont accès à cette fonctionnalité.

La structure de l'objet AuditLog est la suivante :

[inline-code-attrs-start title = 'Structure AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Le journal d'audit est immuable. Il ne peut pas non plus être écrit manuellement. FastComments.com est le seul à décider quand écrire dans le journal d'audit. Cependant, vous pouvez le lire via cette API.

Les événements dans le journal d'audit expirent après deux ans.
