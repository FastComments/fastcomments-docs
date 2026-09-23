ערוך סקר במקום, תוך שמירה על הספירות שלו: שנה את השאלה, שנה תווית של אפשרות, סגור או פתח אותו מחדש,
או שנה מי יכול לראות את המצביעים. אפשרויות מזוהות לפי מזהה - כדי להוסיף, להסיר או לשנות את סדרן, השתמש ב‑PUT של הרשימה המלאה.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| commentPollPatch | CommentPollPatch | כן |  |

## תגובה

מחזיר: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]