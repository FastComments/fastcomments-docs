Сваки agent webhook је потписан помоћу HMAC-SHA256 користећи your tenant's API secret. Иста шема потписивања се користи за FastComments-ове comment webhooks - ако сте их већ интегрисали, agent webhook-ови поново користе исти signature header и verification flow.

### Зашто потписивање

Без потписа, нападач који зна ваш webhook URL би могао послати лажне догађаје који изгледају као да су послати од FastComments. Потписивање значи да ваш endpoint може верификовати свако достављање као аутентично пре него што предузме акцију.

### Како потписи функционишу

За свако достављање:

1. Платформа тражи API secret за tenant + одговарајући domain (види [Преглед вебхукова](#webhooks-overview)).
2. Шаље тренутни Unix timestamp (у милисекундама) у заглављу `X-FastComments-Timestamp`.
3. Израчунава `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (као у Stripe) и резултат шаље као `sha256=<hex>` у заглављу `X-FastComments-Signature`.
4. Ваш endpoint учитава заглавље са timestamp-ом, поново израчунава HMAC над `${timestamp}.${body}` који је примио, упоређује са `sha256=<hex>` вредношћу у signature заглављу и одбацује неусклађености.

Тело које се потписује су **тачни бајтови** које платформа пошаље, са префиксом `${timestamp}.` - ваш верификатор мора користити сирово тело захтева, а не поново сериализовани JSON низ (редослед кључева и размак иначе би се разликовали).

### API secret

The same API Secret used by [comment webhooks](/guide-webhooks.html). It is per (tenant, domain) and managed in your tenant's API settings. If you rotate the secret, you should re-deploy your verifier to read the new value before the next delivery.

Када платформа не пронађе **no API secret** за одговарајући domain, достављање се не дешава. webhook лог бележи неуспјех са разлогом "no API secret".

### Пример верификације (Node.js)

[inline-code-attrs-start title = 'Пример провјере потписа вебхука'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Користите `timingSafeEqual` уместо `===` да бисте избјегли цурење информација о времену везано за потпис.

### Шта се налази у потписаном телу

Пуна коверта плус специфичан за догађај `data` блок. Погледајте [Подаци вебхука](#webhook-payloads).

### Препоруке

- **Проверите при сваком достављању.** Ако ваш endpoint прихвата unsigned requests, немате гаранцију интегритета.
- **Одбаците при неусклађеном потпису.** Вратите 401 или 403; не враћајте 200 OK на лош потпис, иначе ћете прикрити нападе у вашим логовима доставе.
- **Користите HTTPS.** Потписи штите интегритет; TLS штити повјерљивост (и вашу тајну и текст коментара у payload-у).
- **Ротација тајни** када чланови тима са приступом оду, или по распореду.

### Заштита од реплеј напада

Само потписивање не спречава реплеј нападе - нападач који је снимио стварно потписано достављање може га поново послати. Заштита од реплеја је на вашој endpoint страни:

- Користите `occurredAt` поље у коверти и одбаците доставе старије од, рецимо, 5 минута.
- Користите `triggerId` или `approvalId` као dedup кључ - ако сте га већ обрадили, занемарите дупликат.

### Погледајте такође

- [Преглед вебхукова](#webhooks-overview).
- [Подаци вебхука](#webhook-payloads).
- Главни [Webhooks guide](/guide-webhooks.html) за ширу инфраструктуру потписивања.