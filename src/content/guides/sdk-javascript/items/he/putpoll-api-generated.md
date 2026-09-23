צירוף סקר לתגובה, או הגדרת המצב המלא של הסקר שכבר קיים.

אפשרויות מתואמות לפי מזהה: אפשרות שנשלחת עם המזהה של אפשרות קיימת שומרת את הקולות שלה (ולקחת את התווית והמיקום החדשים), אפשרות שנשלחת ללא מזהה מתווספת, והאפשרויות הקיימות שהושמטו מהרשימה מוסרות יחד עם הקולות שהושמעו עליהן.

שמירת אפס מזהי אפשרויות קיימות בסקר שיש לו קולות מוחקת את כולן, ולכן יש צורך ב‑replaceVotes=true.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| commentPollPutInput | CommentPollPutInput | כן |  |
| replaceVotes | boolean | לא |  |

## תגובה

מחזיר: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]