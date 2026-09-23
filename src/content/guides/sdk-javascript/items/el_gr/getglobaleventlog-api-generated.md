req
tenantId
urlId
userIdWS

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| userIdWS | string | Ναι |  |
| startTime | number | Ναι |  |
| endTime | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEventLogs() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const userIdWS: string = "user_ws_abcde";
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // 24 ώρες πριν
  const endTime: number = Date.now();

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
}
[inline-code-end]

---