## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createCommentParams | Array<CreateCommentParams> | 是 |  |
| isLive | boolean | 否 |  |
| doSpamCheck | boolean | 否 |  |
| sendEmails | boolean | 否 |  |
| populateNotifications | boolean | 否 |  |

## 回應

返回： `Array<SaveCommentsBulkResponse`

## 範例

[inline-code-attrs-start title = 'saveCommentsBulk 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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