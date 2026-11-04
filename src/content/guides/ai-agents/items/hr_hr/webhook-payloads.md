Svi webhook payloadi agenata dijele zajedničku omotnicu i dodaju blok `data` specifičan za događaj. Ova stranica navodi potpunu shemu za svaki.

### Omotnica (svaki događaj)

Svaki payload, bez obzira na vrstu događaja, ima ova polja na najvišoj razini:

[inline-code-attrs-start title = 'Shema omotnice webhooka'; type='json' inline-code-attrs-end]
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

Shema `data`:

[inline-code-attrs-start title = 'Shema podataka događaja okidača'; type='json' inline-code-attrs-end]
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

`triggerType` je numerički enum iz [popisa događaja okidača](#triggers-overview).

`actions[].type` je numerički enum iz [popisa alata](#tools-overview).

`actions[].pending` je `true` kada je radnja bila stavljena u red za [odobrenje](#approval-workflow) umjesto izvršena.

### `approval.requested`

Shema `data`:

[inline-code-attrs-start title = 'Shema podataka zahtjeva za odobrenje'; type='json' inline-code-attrs-end]
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

Objekt **`args`** je ono što je poziv LLM alata nosio. Njegov oblik ovisi o alatu:

- Za `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Za `mark_comment_spam`: `{ commentId, isSpam }`.
- Za `write_comment`: `{ comment, urlId, parentId? }`.
- ...i tako dalje.

Skup oblika argumenata alata **nije stabilan javni ugovor**. Alati se mogu dodavati u budućnosti i platforma prosljeđuje argumente nepromijenjene. Potrošači trebaju tretirati args kao neprozirnu masu osim ako izričito ne razumiju uključeni alat.

**`contextSnapshot`** bilježi kontekst komentara, stranice i korisnika iz kojeg je odobrenje stavljeno u red. Njegov oblik zrcali poruku konteksta okidača.

### `approval.decided`

Shema `data`:

[inline-code-attrs-start title = 'Shema podataka odluke o odobrenju'; type='json' inline-code-attrs-end]
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

### Oblik TenantAgentAction

Unutar `actions[]` u payloadima okidača, svaka radnja ima:

[inline-code-attrs-start title = 'Shema TenantAgentAction'; type='json' inline-code-attrs-end]
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

Vrijednosti enuma `type` odgovaraju `AgentActionType`:

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

`SEARCH_MEMORY` se ne pojavljuje u `actions[]` jer je samo za čitanje i ne provjerava se.

### Vrijednosti enuma `triggerType`

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
- 15: `REPLAY` (interno; ne isporučuje se webhookovima)

### Zaglavlja

Svaka isporuka uključuje:

- `X-FastComments-Agent-Event` - kanonski naziv događaja (`trigger.succeeded` itd.).
- `X-FastComments-Signature` - HMAC-SHA256 sirovog tijela koristeći vašu API tajnu. Pogledajte [Potpisivanje webhooka](#webhook-signing).

### Stabilnost

Polja omotnice i dokumentirana polja `data` po događaju dio su javnog ugovora. Dodavanje novih neobaveznih polja postojećim payloadima dopušteno je i ne smatra se promjenom koja lomi kompatibilnost - vaš potrošač treba ignorirati nepoznata polja. Oblik `args` i `contextSnapshot` **nije** dio ugovora.