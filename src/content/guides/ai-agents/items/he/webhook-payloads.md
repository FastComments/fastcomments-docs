כל מטעני ה-webhook של הסוכנים חולקים מעטפת משותפת ומוסיפים בלוק `data` ייחודי לאירוע. עמוד זה מפרט את הסכמה המלאה עבור כל אחד מהם.

### המעטפת (כל אירוע)

כל מטען, ללא קשר לסוג האירוע, מכיל את השדות ברמה העליונה הבאים:

[inline-code-attrs-start title = 'סכמת מעטפת ה-Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - the matched domain for this delivery",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 timestamp",
  "data": { /* event-specific, see below */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

סכמת `data`:

[inline-code-attrs-start title = 'סכמת נתוני אירוע טריגר'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "triggerId": "string",
  "triggerType": 0,
  "status": "SUCCESS | ERROR",
  "tokensUsed": 1234,
  "wasDryRun": false,
  "actions": [
    {
      "type": 0,
      "commentId": "string - optional",
      "userId": "string - optional",
      "badgeId": "string - optional",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - present on trigger.failed",
  "url": "string - optional",
  "urlId": "string - optional",
  "commentId": "string - optional"
}
[inline-code-end]

`triggerType` הוא enum מספרי מתוך [רשימת אירועי הטריגר](#triggers-overview).

`actions[].type` הוא enum מספרי מתוך [רשימת הכלים](#tools-overview).

`actions[].pending` הוא `true` כאשר הפעולה הועמדה בתור ל[אישור](#approval-workflow) במקום לבוצע.

### `approval.requested`

סכמת `data`:

[inline-code-attrs-start title = 'סכמת נתוני בקשת אישור'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* per-tool, see below */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optional, agent reasoning",
  "confidence": 0.85,
  "contextSnapshot": { /* the comment/page context the approval is about */ }
}
[inline-code-end]

האובייקט **`args`** הוא כל מה שקריאת הכלי של ה-LLM העבירה. צורתו תלויה בכלי:

- עבור `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- עבור `mark_comment_spam`: `{ commentId, isSpam }`.
- עבור `write_comment`: `{ comment, urlId, parentId? }`.
- ...וכן הלאה.

קבוצת צורות הארגומנטים של הכלים **אינה חוזה ציבורי יציב**. כלים יכולים להתווסף בעתיד והפלטפורמה מעבירה את הארגומנטים כפי שהם. צרכנים צריכים להתייחס ל-args כאל בלוק אטום, אלא אם כן הם מבינים במפורש את הכלי המעורב.

ה-**`contextSnapshot`** לוכד את הקשר התגובה, העמוד והמשתמש שממנו האישור הועמד בתור. צורתו משקפת את הודעת ההקשר של הטריגר.

### `approval.decided`

סכמת `data`:

[inline-code-attrs-start title = 'סכמת נתוני החלטת אישור'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - the userId of the moderator who decided",
  "decidedAt": "string - ISO 8601 - optional, only present once decided",
  "executedAt": "string - ISO 8601 - present when APPROVED and execution finished",
  "executionResult": "string - executor result message - present after execute",
  "contextSnapshot": { /* same as approval.requested */ }
}
[inline-code-end]

### צורת TenantAgentAction

בתוך `actions[]` במטעני הטריגר, לכל פעולה יש:

[inline-code-attrs-start title = 'סכמת TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - optional",
  "userId": "string - optional",
  "badgeId": "string - optional",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

ערכי ה-enum של `type` תואמים ל-`AgentActionType`:

- 0: `WRITE_COMMENT`
- 1: `VOTE_COMMENT`
- 2: `PIN_COMMENT`
- 3: `UNPIN_COMMENT`
- 4: `LOCK_COMMENT`
- 5: `UNLOCK_COMMENT`
- 6: `MARK_COMMENT_REVIEWED`
- 7: `MARK_COMMENT_APPROVED`
- 8: `MARK_COMMENT_SPAM`
- 9: `AWARDED_BADGE`
- 10: `BAN_USER`
- 11: `SENT_EMAIL`
- 12: `WARNED_USER`
- 13: `SAVED_MEMORY`

`SEARCH_MEMORY` אינו מופיע ב-`actions[]` מכיוון שהוא לקריאה בלבד ואינו מבוקר.

### ערכי enum של `triggerType`

`AgentTriggerReasonType`:

- 0: `COMMENT_ADD`
- 1: `COMMENT_EDIT`
- 2: `COMMENT_DELETE`
- 3: `COMMENT_PIN`
- 4: `COMMENT_UNPIN`
- 5: `COMMENT_LOCK`
- 6: `COMMENT_UNLOCK`
- 7: `COMMENT_VOTE_THRESHOLD`
- 8: `MODERATOR_REVIEWED_COMMENT`
- 9: `MODERATOR_APPROVED_COMMENT`
- 10: `MODERATOR_SPAMMED_COMMENT`
- 11: `MODERATOR_AWARDED_BADGE`
- 12: `COMMENT_FLAG_THRESHOLD`
- 13: `NEW_USER_FIRST_COMMENT`
- 14: `COMMENT_AUTO_SPAMMED`
- 15: `REPLAY` (פנימי; אינו נשלח ל-webhooks)

### כותרות (Headers)

כל משלוח כולל:

- `X-FastComments-Agent-Event` - שם האירוע הקנוני (`trigger.succeeded` וכו').
- `X-FastComments-Signature` - HMAC-SHA256 של גוף הבקשה הגולמי באמצעות סוד ה-API שלך. ראה [חתימת Webhook](#webhook-signing).

### יציבות

שדות המעטפת ושדות ה-`data` המתועדים עבור כל אירוע הם חלק מהחוזה הציבורי. הוספת שדות אופציונליים חדשים למטענים קיימים מותרת ואינה נחשבת שינוי שובר תאימות - הצרכן שלך צריך להתעלם משדות לא מוכרים. צורת ה-`args` וה-`contextSnapshot` **אינה** חלק מהחוזה.