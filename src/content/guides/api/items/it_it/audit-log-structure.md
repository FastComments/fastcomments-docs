An `AuditLog` è un oggetto che rappresenta un evento auditato per i tenant che hanno accesso a questa funzionalità.

La struttura dell'oggetto AuditLog è la seguente:

[inline-code-attrs-start title = 'Struttura AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` e `targetLabel` descrivono su cosa è stato eseguito l'evento; `userId` e `username` descrivono chi lo ha eseguito. Per gli aggiornamenti, `objectDetails.changes` contiene una mappa `{field: {from, to}}` di ciò che è effettivamente cambiato.

Il registro di audit è immutabile. Non può nemmeno essere scritto manualmente. FastComments.com può decidere solo quando scrivere nel registro di audit. Tuttavia, è possibile leggerlo tramite questa API.

Gli eventi nel registro di audit scadono dopo due anni.