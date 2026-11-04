Svi agent webhook payload-i dele zajedničku omotnicu i dodaju blok `data` specifičan za događaj. Ova stranica navodi kompletnu šemu za svaki.

### Omotnica (svaki događaj)

Svaki payload, bez obzira na tip događaja, ima ove polja na najvišem nivou:

[inline-code-attrs-start title = 'Šema omotnice webhook-a'; type='json' inline-code-attrs-end]
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

Šema `data`:

[inline-code-attrs-start title = 'Šema podataka događaja okidača'; type='json' inline-code-attrs-end]
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

`triggerType` je numerička enumeracija iz [liste događaja okidača](#triggers-overview).

`actions[].type` je numerička enumeracija iz [liste alata](#tools-overview).

`actions[].pending` je `true` kada je akcija stavljena u red za [odobravanje](#approval-workflow) umesto da se izvrši.

### `approval.requested`

Šema `data`:

[inline-code-attrs-start title = 'Šema podataka zahteva za odobravanje'; type='json' inline-code-attrs-end]
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

Objekat **`args`** je ono što je LLM poziv alata nosio. Njegov oblik zavisi od alata:

- Za `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Za `mark_comment_spam`: `{ commentId, isSpam }`.
- Za `write_comment`: `{ comment, urlId, parentId? }`.
- ...i tako dalje.

Skup oblika argumenata alata **nije stabilan javni ugovor**. Alati mogu biti dodati u budućnosti i platforma prosleđuje argumente netaknute. Potrošači bi trebalo da tretiraju argumente kao neproziran blok podataka osim ako eksplicitno ne razumeju u pitanju alat.

**`contextSnapshot`** beleži kontekst komentara, stranice i korisnika iz kog je odobravanje stavljeno u red. Njegov oblik odražava kontekstualnu poruku okidača.

### `approval.decided`

Šema `data`:

[inline-code-attrs-start title = 'Šema podataka odluke o odobravanju'; type='json' inline-code-attrs-end]
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

Unutar `actions[]` u payload-ima okidača, svaka akcija ima:

[inline-code-attrs-start title = 'Šema TenantAgentAction'; type='json' inline-code-attrs-end]
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

Vrednosti enumeracije `type` odgovaraju `AgentActionType`:

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

`SEARCH_MEMORY` se ne pojavljuje u `actions[]` jer je samo za čitanje i ne beleži se u evidenciju.

### Vrednosti enumeracije `triggerType`

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
- 15: `REPLAY` (interno; ne dostavlja se webhook-ovima)

### Zaglavlja

Svaka dostava uključuje:

- `X-FastComments-Agent-Event` - kanonski naziv događaja (`trigger.succeeded`, itd.).
- `X-FastComments-Signature` - HMAC-SHA256 sirovog tela koristeći vašu API tajnu. Pogledajte [Potpisivanje webhook-ova](#webhook-signing).

### Stabilnost

Polja omotnice i dokumentovana polja `data` po događaju deo su javnog ugovora. Dodavanje novih opcionih polja postojećim payload-ima je dozvoljeno i ne smatra se promenom koja kini kompatibilnost - vaš potrošač treba da ignoriše nepoznata polja. Oblik `args` i `contextSnapshot` **nije** deo ugovora.