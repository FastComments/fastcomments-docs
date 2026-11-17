req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| page | number | नहीं |  |
| direction | SortDirections | नहीं |  |
| sso | string | नहीं |  |
| skip | number | नहीं |  |
| skipChildren | number | नहीं |  |
| limit | number | नहीं |  |
| limitChildren | number | नहीं |  |
| countChildren | boolean | नहीं |  |
| fetchPageForCommentId | string | नहीं |  |
| includeConfig | boolean | नहीं |  |
| countAll | boolean | नहीं |  |
| includei10n | boolean | नहीं |  |
| locale | string | नहीं |  |
| modules | string | नहीं |  |
| isCrawler | boolean | नहीं |  |
| includeNotificationCount | boolean | नहीं |  |
| asTree | boolean | नहीं |  |
| maxTreeDepth | number | नहीं |  |
| useFullTranslationIds | boolean | नहीं |  |
| parentId | string | नहीं |  |
| searchText | string | नहीं |  |
| hashTags | Array<string> | नहीं |  |
| userId | string | नहीं |  |
| customConfigStr | string | नहीं |  |
| afterCommentId | string | नहीं |  |
| beforeCommentId | string | नहीं |  |
| questionId | string | नहीं |  |
| questionValue | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';

const response: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
  tenantId,
  urlId,
  1,
  'DESC' as SortDirections,
  undefined,
  0,
  undefined,
  25,
  undefined,
  true,
  undefined,
  undefined,
  true,
  true,
  false,
  'en-US',
  undefined,
  false,
  true,
  undefined,
  true,
  3,
  undefined,
  undefined,
  'root',
  'authentication',
  ['announcement', 'qna'],
  undefined,
  undefined,
  undefined,
  undefined,
  undefined
);

const comments: PublicComment[] = response.comments;
const presenceCount: number = response.presence?.userCount ?? 0;
console.log(`Fetched ${comments.length} comments, ${presenceCount} users online`);
[inline-code-end]