Obiekt `AuditLog` reprezentuje zdarzenie podlegające audytowi dla tenantów, którzy mają dostęp do tej funkcji.

Struktura obiektu AuditLog wygląda następująco:

[inline-code-attrs-start title = 'Struktura obiektu AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Dziennik audytu jest niezmienny. Nie można też zapisywać do niego ręcznie. FastComments.com decyduje jedynie, kiedy zapisywać wpisy w dzienniku audytu. Możesz jednak odczytywać go za pomocą tego API.

Wpisy w dzienniku audytu wygasają po dwóch latach.