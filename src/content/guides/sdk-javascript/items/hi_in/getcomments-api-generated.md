## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| page | number | नहीं |  |
| limit | number | नहीं |  |
| skip | number | नहीं |  |
| asTree | boolean | नहीं |  |
| skipChildren | number | नहीं |  |
| limitChildren | number | नहीं |  |
| maxTreeDepth | number | नहीं |  |
| urlId | string | नहीं |  |
| userId | string | नहीं |  |
| anonUserId | string | नहीं |  |
| contextUserId | string | नहीं |  |
| hashTag | string | नहीं |  |
| parentId | string | नहीं |  |
| direction | SortDirections | नहीं |  |
| fromDate | number | नहीं |  |
| toDate | number | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getComments उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "acme-corp";
  const page: number = 1;
  const limit: number = 50;
  const asTree: boolean = false;
  const direction: SortDirections = "asc";
  const fromDate: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 दिन पहले
  const toDate: number = Date.now();

  const commentsResponse: APIGetCommentsResponse = await getComments(
    tenantId,
    page,
    limit,
    undefined,
    asTree,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    direction,
    fromDate,
    toDate
  );

  console.log(commentsResponse);
}
[inline-code-end]