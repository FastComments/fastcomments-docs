Svaki agent webhook има свој записник испоруке. Доступан је са [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) путем **Logs** дугмета на сваком реду webhook-а.

### Шта се налази на страници

Пагинирана табела са по једним редом по покушају испоруке:

| Column | Meaning |
|---|---|
| When | Kada se pokušaj dogodio. |
| Event | Tip događaja (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | Статус испоруке. |
| StatusCode | HTTP status code returned by your endpoint, when available. |
| Description | Kratak opis rezultata. Za neuspjele isporuke gdje nije primljen HTTP odgovor, osnovna poruka o grešci se čuva kao `{error: <message>}`. |

Stranica podržava samo paginaciju - нема филтера по статусу, типу догађаја или распону датума.

### Шта можете урадити из записника

- **Одлучите да ли статусни код треба да буде у No-retry.** Ако видите да ваш endpoint враћа исти `4xx` изнова и изнова, то је јак сигнал да желите да га додате у **No-retry status codes** тако да платформа престане са поновним покушајима.

### Информације о неуспјеху

Када испорука не успије без HTTP одговора (DNS failure, connection refused, timeout, TLS error, итд.), сировa порука о грешци се евидентира као `{error: <message>}`. Платформа их не категорише у именоване групе као `TIMEOUT` или `DNS_ERROR` - прочитајте поруку о грешци директно да видите шта се десило.

За HTTP одговоре, колона StatusCode приказује код који је вратио ваш endpoint. Уобичајени случајеви:

- **All attempts: `401` or `403`** - ваш endpoint одбацује потпис. Провјерите да ли рачунате HMAC преко `${timestamp}.${body}` и користите прави тајни кључ. Погледајте [Webhook Signing](#webhook-signing).
- **All attempts: `422`** - ваш endpoint мисли да је payload неважећи. Или исправите ваш endpoint, или додате `422` у **No-retry status codes** ако је одбацивање очекивано за неке догађаје.

### Контекст по испоруци

Сваки унос у записнику садржи:

- `webhookId` - која конфигурација webhook-а је произвела ову испоруку.
- `agentId` - о којем агенту се испорука тиче.
- `triggerId` or `approvalId` - основни запис.
- `domain` - подударени домен.

Можете их користити да корелирате неуспјелу испоруку са покретањем на које се односи у [Run History](#run-history).

### Задржавање

`AgentSyncLog` уноси имају једногодишњи TTL на `createdAt` без обзира на исход - успјешне и неуспјеле испоруке се чувају исто толико времена. По истеку рока чувања, унос у записнику нестаје.

Ако вам треба дугорочни ревизорски запис, одржив образац је: нека сам **endpoint** перзистира испоруке које прими. То раздваја ваш ревизорски запис од политике чувања платформе.

### Тестно слање

Дугме **Test send** у форми за конфигурацију webhook-а уписује лажну испоруку у исту табелу записника тако да можете провјерити повезаност од краја до краја прије ослањања на стварне догађаје. Тестне испоруке су јасно означене у записнику тако да не загађују продукцијску статистику грешака.

### Погледајте и

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) за семантику поновних покушаја која стоји иза ових записника.
- [Webhook Signing](#webhook-signing) за начин провјере на вашем endpoint-у.