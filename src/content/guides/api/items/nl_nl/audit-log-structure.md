Een `AuditLog` is een object dat een geaudit evenement vertegenwoordigt voor huurders die toegang hebben tot deze functie.

De structuur voor het AuditLog-object is als volgt:

[inline-code-attrs-start title = 'AuditLog Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` en `targetLabel` beschrijven waarop het evenement werd uitgevoerd; `userId` en `username` beschrijven wie het heeft uitgevoerd. Voor updates bevat `objectDetails.changes` een `{field: {from, to}}` kaart van wat daadwerkelijk is veranderd.

Het auditlogboek is onveranderlijk. Het kan ook niet handmatig worden geschreven. FastComments.com mag alleen beslissen wanneer er naar het auditlogboek wordt geschreven. U kunt er echter via deze API uit lezen.

Evenementen in het auditlogboek verlopen na twee jaar.