req
tenantId
urlId
userIdWS

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| userIdWS | string | Ναι |  |
| startTime | number | Ναι |  |
| endTime | number | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-84b2f1";
const urlId: string = "article-6721";
const userIdWS: string = "ws-conn-9a3c";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 7 ημέρες πριν
const endTimeOptional: number | undefined = undefined; // προαιρετικό τέλος χρονικού διαστήματος
const endTime: number = endTimeOptional ?? Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---