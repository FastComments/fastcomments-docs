## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | Array<CreateCommentParams> | 예 |  |
| isLive | boolean | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| sendEmails | boolean | 아니오 |  |
| populateNotifications | boolean | 아니오 |  |

## 응답

반환: `Array<SaveCommentsBulkResponse`

## 예시

[inline-code-attrs-start title = 'saveCommentsBulk 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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