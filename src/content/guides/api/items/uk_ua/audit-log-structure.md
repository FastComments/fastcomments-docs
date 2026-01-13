Об'єкт `AuditLog` представляє подію, що підлягала аудитові, для орендарів, які мають доступ до цієї функції.

Структура об'єкта `AuditLog` має такий вигляд:

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

Журнал аудиту є незмінним. Також у нього не можна записувати вручну. Тільки FastComments.com може вирішувати, коли записувати до журналу аудиту. Однак ви можете читати його через цей API.

Записи в журналі аудиту зберігаються протягом двох років.