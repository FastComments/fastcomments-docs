Svi webhook payloadovi agenta dijele zajednički omot i dodaju event-specifičan `data` blok. Ova stranica navodi potpunu shemu za svaki.

### Omot (svaki događaj)

Svaka poruka, bez obzira na tip događaja, ima ova vršna polja:

[inline-code-attrs-start title = 'Shema omota webhooka'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - podudarna domena za ovu isporuku",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 vremenska oznaka",
  "data": { /* specifično za događaj, vidi dolje */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` shema:

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
      "commentId": "string - neobavezno",
      "userId": "string - neobavezno",
      "badgeId": "string - neobavezno",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - prisutno u trigger.failed",
  "url": "string - neobavezno",
  "urlId": "string - neobavezno",
  "commentId": "string - neobavezno"
}
[inline-code-end]

`triggerType` je numerički enum iz [popisa događaja okidača](#triggers-overview).

`actions[].type` je numerički enum iz [popisa alata](#tools-overview).

`actions[].pending` je `true` kada je akcija stavljena u čekanje za [odobrenje](#approval-workflow) umjesto da je izvršena.

### `approval.requested`

`data` shema:

[inline-code-attrs-start title = 'Shema podataka zahtjeva za odobrenjem'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* ovisno o alatu, vidi dolje */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - neobavezno, obrazloženje agenta",
  "confidence": 0.85,
  "contextSnapshot": { /* kontekst komentara/stranice na koji se odnosi odobrenje */ }
}
[inline-code-end]

Objekt **`args`** je što god je LLM poziv alata prenio. Njegov oblik ovisi o alatu:

- Za `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Za `mark_comment_spam`: `{ commentId, isSpam }`.
- Za `write_comment`: `{ comment, urlId, parentId? }`.
- ...i tako dalje.

Skup oblika argumenata alata nije stabilan javni ugovor. Alati se mogu dodavati u budućnosti, a platforma prosljeđuje args bez promjena. Potrošači bi trebali tretirati args kao nečitljiv blob osim ako izričito ne razumiju uključeni alat.

**`contextSnapshot`** bilježi kontekst komentara, stranice i korisnika iz kojeg je odobrenje stavljeno u čekanje. Njegov oblik odražava kontekst poruke okidača.

### `approval.decided`

`data` shema:

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

### TenantAgentAction shape

Unutar `actions[]` u trigger payloadovima, svaka akcija ima:

[inline-code-attrs-start title = 'Shema TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - neobavezno",
  "userId": "string - neobavezno",
  "badgeId": "string - neobavezno",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` vrijednosti enuma odgovaraju `AgentActionType`:

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

`SEARCH_MEMORY` se ne pojavljuje u `actions[]` zato što je samo za čitanje i nije revidiran.

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

- `X-FastComments-Agent-Event` - kanoničko ime događaja (`trigger.succeeded`, itd.).
- `X-FastComments-Signature` - HMAC-SHA256 sirovog tijela koristeći vaš API tajni ključ. Vidi [Potpisivanje webhooka](#webhook-signing).

### Stabilnost

Polja omota i dokumentirana `data` polja po događaju dio su javnog ugovora. Dodavanje novih opcionalnih polja postojećim payloadovima je dopušteno i ne smatra se breaking promjenom - vaš potrošač bi trebao ignorirati nepoznata polja. Oblik `args` i `contextSnapshot` **nije** dio ugovora.

---