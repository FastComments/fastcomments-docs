`AuditLog` е обект, който представлява одитирано събитие за наематели, които имат достъп до тази функция.

Структурата на обекта AuditLog е следната:

[inline-code-attrs-start title = 'Структура на AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Одитният дневник е неизменяем. Също така не може да се записва ръчно. Само FastComments.com може да реши кога да записва в одитния дневник. Въпреки това можете да четете от него чрез този API.

Събитията в одитния дневник изтичат след две години.
