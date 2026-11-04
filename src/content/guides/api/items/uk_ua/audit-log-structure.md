`AuditLog` — це об'єкт, який представляє аудитову подію для орендарів, що мають доступ до цієї функції.

Структура об'єкта AuditLog виглядає наступним чином:

[inline-code-attrs-start title = 'Структура AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Хто виконав подію. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Браузер, з якого виконано подію, якщо вона надійшла з нього. **/
    ua?: string;
    /** Хеш сесії, з якої надійшла подія, для співвіднесення дій однієї людини. Ніколи не містить саму сесію. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** Ідентифікатор об'єкта, над яким виконано подію, на відміну від того, хто її виконав. **/
    targetId?: string;
    /** Зрозуміла для людини мітка цього об'єкта, наприклад "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` і `targetLabel` описують, над чим було виконано подію; `userId` і `username` описують, хто її виконав. Для оновлень `objectDetails.changes` містить карту `{field: {from, to}}` того, що фактично змінилося.

Журнал аудиту є незмінним. Його також не можна записувати вручну. Лише FastComments.com може вирішувати, коли здійснювати запис до журналу аудиту. Однак ви можете читати з нього через цей API.

Події в журналі аудиту спливають через два роки.