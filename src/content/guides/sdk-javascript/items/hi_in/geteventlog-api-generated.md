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

लौटाता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getEventLog उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "url_9876";
const userIdWS: string = "user_abcde";
const startTime: number = Date.now() - 86_400_000; // 24 घंटे पहले
const endTime: number = Date.now();

const fullLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const partialLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]