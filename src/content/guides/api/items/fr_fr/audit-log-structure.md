Un `AuditLog` est un objet qui représente un événement audité pour les locataires qui ont accès à cette fonctionnalité.

La structure de l'objet AuditLog est la suivante :

[inline-code-attrs-start title = 'Structure d\'AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Who performed the event. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** The browser that performed the event, when it came from one. **/
    ua?: string;
    /** A hash of the session the event came from, for correlating one person's actions. Never the session itself. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** The id of the object the event was performed on, as opposed to who performed it. **/
    targetId?: string;
    /** A human-readable label for that object, e.g. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` et `targetLabel` décrivent sur quoi l'événement a été effectué ; `userId` et `username` décrivent qui l'a effectué. Pour les mises à jour, `objectDetails.changes` contient une carte `{field: {from, to}}` de ce qui a réellement changé.

Le journal d'audit est immuable. Il ne peut pas non plus être écrit manuellement. FastComments.com peut uniquement décider quand écrire dans le journal d'audit. Cependant, vous pouvez le lire via cette API.

Les événements du journal d'audit expirent après deux ans.