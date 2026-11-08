दस्तावेज़ों को समूहित करके (यदि groupBy दिया गया है) और कई ऑपरेशन लागू करके एग्रीगेट करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| aggregationRequest | AggregationRequest | हाँ |  |
| parentTenantId | string | नहीं |  |
| includeStats | boolean | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'aggregate उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-12345";

const aggregationRequest: AggregationRequest = {
  predicates: [
    {
      field: "status",
      operator: "eq",
      value: { type: "string", value: "approved" }
    }
  ],
  operations: [
    { type: "count", field: "commentId" }
  ],
  sort: { field: "createdAt", direction: "desc" }
};

const parentTenantId: string = "parent-001";
const includeStats: boolean = true;

const result: AggregateResponse = await aggregate(
  tenantId,
  aggregationRequest,
  parentTenantId,
  includeStats
);
[inline-code-end]