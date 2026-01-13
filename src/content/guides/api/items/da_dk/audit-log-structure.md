Et `AuditLog` er et objekt, der repræsenterer en revideret hændelse for lejere, der har adgang til denne funktion.

Strukturen for AuditLog-objektet er som følger:

[inline-code-attrs-start title = 'AuditLog-struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Auditloggen er uforanderlig. Den kan heller ikke skrives til manuelt. FastComments.com beslutter alene, hvornår der skrives til auditloggen. Du kan dog læse fra den via dette API.

Hændelser i auditloggen udløber efter to år.