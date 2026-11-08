req
tenantId
urlId
userIdWS

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| userIdWS | string | हाँ |  |
| startTime | number | हाँ |  |
| endTime | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getGlobalEventLog उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEventLogs() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const userIdWS: string = "user_ws_abcde";
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // 24 घंटे पहले
  const endTime: number = Date.now();

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
}
[inline-code-end]