Webhooks се такође могу управљати преко REST API‑а. Овако интеграције попут Zapier‑а претплаћују се на догађаје коментара без коришћења контролне табле, и следе образац REST Hooks: претплата, примање догађаја, отказивање претплате.

API претплате постоје заједно са webhook‑овима подешеним у контролној табли. Догађај коментара се испоручује сваком webhook‑у који одговара његовом домену, сваки као посебна испорука, без обзира како је webhook креиран.

## Аутентикација

Сваки захтев захтева ваш API кључ у заглављу `x-api-key` (или у параметру упита `API_KEY`) и ваш ID закупца у параметру упита `tenantId`. Оба су приказана на страници API Secret у контролној табли.

## Претплата

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Поље | Обавезно | Опис |
|-------|----------|-------------|
| `url` | Да | Апсолутни http или https URL. |
| `event` | Да | `comment-created`, `comment-updated` или `comment-deleted`. |
| `domain` | Не | Домен из конфигурације вашег налога. Подразумевано је `*`, који прима догађаје за сваки домен. |
| `method` | Не | `POST` (подразумевано), `PUT` или `DELETE`. |

Одговор садржи претплату:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Претплата истог URL‑а на исти догађај и домен поново враћа постојећу претплату уместо креирања дупликата, тако да клијент може безбедно поново покушати. Сваки закупац може имати до 50 API претплата.

## Листа

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Враћа све webhook‑ове за закупца, укључујући оне управљане у контролној табли (`"source": "dashboard"`). Филтрирајте помоћу `event`, `domain` или `source`.

## Откажи претплату

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Брисање претплате такође одбацује све догађаје који су још у реду за њу. Само претплате креиране преко API‑а могу се брисати на овај начин. Webhook‑ови у контролној табли се уређују на страници Webhooks.

## Тела захтева и потписивање

Испоруке користе исти payload као webhook‑ови у контролној табли (види Data Structures) и потписани су истим HMAC шемом (види Security & API Tokens). API претплате никада не примају старо `token` заглавље, па уместо тога проверите заглавље `X-FastComments-Signature`.

## Пример тела захтева

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Враћа најновије коментаре налога у тачно истом облику у коме их испорука носи, тако да интеграција може приказати прави пример података пре доласка првог догађаја. `event` је опционо и само се верификује, јер сваки догађај испоручује исти објекат коментара. `limit` подразумевано је 3 и прихвата вредности од 1 до 10. Потрошња је 2 API кредита.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Одговор са 410 Gone

Ако крајња тачка API претплате одговори HTTP статусом `410 Gone`, FastComments то сматра отказом претплате: претплата се брише заједно са догађајима у реду, и даље испоруке се не покушавају. Webhook‑ови подешени у контролној табли се никада аутоматски не бришу; за њих 410 представља обичан неуспех. Сваки други статус неуспеха се поново покушава и на крају онемогућава webhook, као што је описано у How it Works & Handling Retries.

## Контролна табла

API претплате се појављују у листи Webhooks са извором **API**, где администратор може да их уређује, онемогући, поново омогући или избрише.