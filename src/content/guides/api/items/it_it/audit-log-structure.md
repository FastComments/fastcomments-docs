---
Un `AuditLog` è un oggetto che rappresenta un evento sottoposto ad audit per i tenant che hanno accesso a questa funzionalità.

La struttura dell'oggetto AuditLog è la seguente:

[inline-code-attrs-start title = 'Struttura di AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Il registro di audit è immutabile. Inoltre non può essere scritto manualmente. Solo FastComments.com può decidere quando scrivere nel registro di audit. Tuttavia, è possibile leggerlo tramite questa API.

Gli eventi nel registro di audit scadono dopo due anni.

---