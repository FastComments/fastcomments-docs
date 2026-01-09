---
`AuditLog` — объект, представляющий аудируемое событие для арендаторов, имеющих доступ к этой функции.

Структура объекта AuditLog выглядит следующим образом:

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

Журнал аудита неизменяем. В него также нельзя записывать вручную. FastComments.com может принимать решение только о том, когда записывать в журнал аудита. Тем не менее, вы можете считывать его через этот API.

События в журнале аудита истекают через два года.

---