## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createCommentParams | Array<CreateCommentParams> | はい |  |
| isLive | boolean | いいえ |  |
| doSpamCheck | boolean | いいえ |  |
| sendEmails | boolean | いいえ |  |
| populateNotifications | boolean | いいえ |  |

## Response

戻り値: `Array<SaveCommentsBulkResponse`

## Example

[inline-code-attrs-start title = 'saveCommentsBulk の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const mention1: CommentUserMentionInfo = {
  userId: "user_002",
  start: 10,
  end: 20,
};

const hashtag1: CommentUserHashTagInfo = {
  tag: "news",
  start: 30,
  end: 35,
};

const poll1: CommentPollInput = {
  question: "Did you find this helpful?",
  options: ["Yes", "No"],
};

const commentA: CreateCommentParams = {
  content: "This is a great article!",
  userId: "user_001",
  mentions: [mention1],
  hashtags: [hashtag1],
  poll: poll1,
};

const commentB: CreateCommentParams = {
  content: "I have a question about the topic.",
  userId: "user_003",
};

const createCommentParams: CreateCommentParams[] = [commentA, commentB];

const isLive: boolean = true;
const doSpamCheck: boolean = false;
const sendEmails: boolean = true;
const populateNotifications: boolean = false;

const results: SaveCommentsBulkResponse[] = await saveCommentsBulk(
  tenantId,
  createCommentParams,
  isLive,
  doSpamCheck,
  sendEmails,
  populateNotifications
);
[inline-code-end]