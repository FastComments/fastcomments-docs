## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| userId | string | नहीं |  |
| direction | SortDirections | नहीं |  |
| repliesToUserId | string | नहीं |  |
| page | number | नहीं |  |
| includei10n | boolean | नहीं |  |
| locale | string | नहीं |  |
| isCrawler | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsForUser उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const direction: SortDirections = SortDirections.Descending;
const page: number = 3;
const includei10n: boolean = true;
const locale: string = 'fr-FR';
const isCrawler: boolean = false;

const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(
  userId,
  direction,
  undefined,
  page,
  includei10n,
  locale,
  isCrawler
);
[inline-code-end]