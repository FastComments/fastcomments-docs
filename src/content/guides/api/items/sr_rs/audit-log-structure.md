An `AuditLog` је објекат који представља ревидиран догађај за закупце који имају приступ овој функцији.

Структура објекта AuditLog је следећа:

[inline-code-attrs-start title = 'Структура AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Ко је извршио догађај. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** Претраживач који је извршио догађај, када је дошло из једног. **/
    ua?: string;
    /** Хеш сесије из које је догађај дошао, за повезивање радњи једне особе. Никада сама сесија. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** ИД објекта на коме је догађај извршен, уместо ко је извршио. **/
    targetId?: string;
    /** Човеку читљиви етикет за тај објекат, нпр. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` и `targetLabel` описују на чему је догађај извршен; `userId` и `username` описују ко је извршио догађај. За ажурирања, `objectDetails.changes` садржи мапу `{field: {from, to}}` која приказује шта је заиста промењено.

Списак ревизија је неизмењив. Такође се не може ручно уписивати. FastComments.com може само одлучити када ће уписати у списак ревизија. Међутим, можете га читати преко овог API‑ја.

Догађаји у списку ревизија истичу након две године.