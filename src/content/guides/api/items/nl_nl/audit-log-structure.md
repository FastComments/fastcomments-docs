Een `AuditLog` is een object dat een geaudit evenement vertegenwoordigt voor huurders die toegang hebben tot deze functie.

De structuur voor het AuditLog-object is als volgt:

[inline-code-attrs-start title = 'AuditLog-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Het auditlogboek is onveranderlijk. Het kan ook niet handmatig worden geschreven. FastComments.com bepaalt alleen wanneer er naar het auditlogboek wordt geschreven. U kunt er echter wel uit lezen via deze API.

Gebeurtenissen in het auditlogboek vervallen na twee jaar.