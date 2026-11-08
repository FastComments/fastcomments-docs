## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | नहीं |  |
| state | number | नहीं |  |
| skip | number | नहीं |  |
| limit | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getTickets उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const ticketsSimple: GetTicketsResponse = await getTickets(tenantId);

  const userId: string = "user_9876";
  const state: number = 1; // उदाहरण के लिए, खुला
  const skip: number = 0;
  const limit: number = 20;
  const ticketsFull: GetTicketsResponse = await getTickets(
    tenantId,
    userId,
    state,
    skip,
    limit
  );
})();
[inline-code-end]