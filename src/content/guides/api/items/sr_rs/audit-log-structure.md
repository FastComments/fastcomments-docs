А `AuditLog` је објекат који представља ревидирани догађај за тенанте који имају приступ овој функцији.

Структура објекта AuditLog је следећа:

[inline-code-attrs-start title = 'Структура AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Аудит лог је неменљив. Такође му се не може ручно писати. FastComments.com може једино одлучити када ће уписати у аудит лог. Међутим, ви можете читати из њега преко овог API-ја.

Догађаји у аудит логу истичу након две године.