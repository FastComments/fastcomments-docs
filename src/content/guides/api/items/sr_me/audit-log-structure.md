`AuditLog` je objekat koji predstavlja revizirani događaj za tenantima koji imaju pristup ovoj funkciji.

Struktura AuditLog objekta je sljedeća:

[inline-code-attrs-start title = 'Struktura AuditLog objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Dnevnik revizije je nepromjenjiv. Ne može se ručno upisivati. FastComments.com jedino može odlučiti kada će unijeti zapise u dnevnik revizije. Međutim, možete ga čitati putem ovog API-ja.

Događaji u dnevniku revizije ističu nakon dvije godine.