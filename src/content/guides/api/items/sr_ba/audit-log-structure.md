`AuditLog` је објекат који представља ревидиран догађај за тенанте који имају приступ овој функцији.

Структура за AuditLog објекат је следећа:

[inline-code-attrs-start title = 'Структура AuditLog-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Дневник ревизија је непроменљив. У њега се такође не може ручно уписивати. Само FastComments.com може одлучити када ће уписати у дневник ревизија. Међутим, ви можете читати из њега преко овог API-ја.

Записи у дневнику ревизија истичу након двије године.