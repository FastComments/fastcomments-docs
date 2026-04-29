Svi agent webhook payload-ovi dele zajednički omotač i dodaju blok `data` specifičan za događaj. Ova stranica navodi punu šemu za svaki od njih.

### Omotač (svaki događaj)

Svaki payload, bez obzira na tip događaja, sadrži ova polja na najvišem nivou:

[inline-code-attrs-start title = 'Šema Webhook omotača'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - odgovarajući domen za ovu isporuku",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 vremenska oznaka",
  "data": { /* specifično za događaj, vidi dole */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` šema:

[inline-code-attrs-start title = 'Šema podataka trigger događaja'; type='json' inline-code-attrs-end]
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
  "errorMessage": "string - prisutan kod trigger.failed",
  "url": "string - neobavezno",
  "urlId": "string - neobavezno",
  "commentId": "string - neobavezno"
}
[inline-code-end]

`triggerType` je numerički enum iz [liste trigger događaja](#triggers-overview).

`actions[].type` je numerički enum iz [liste alata](#tools-overview).

`actions[].pending` je `true` kada je akcija stavljena u red za [odobrenje](#approval-workflow) umesto da je izvršena.

### `approval.requested`

`data` šema:

[inline-code-attrs-start title = 'Šema podataka zahteva za odobrenje'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* specifično za alat, vidi dole */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - neobavezno, obrazloženje agenta",
  "confidence": 0.85,
  "contextSnapshot": { /* kontekst komentara/stranice na koji se odnosi odobrenje */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- Za `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Za `mark_comment_spam`: `{ commentId, isSpam }`.
- Za `write_comment`: `{ comment, urlId, parentId? }`.
- ...i tako dalje.

Skup oblika argumenata alata nije **stabilan javni ugovor**. Alati mogu biti dodati u budućnosti i platforma prosleđuje args verbatim. Potrošači bi trebali tretirati args kao neprovidan blob osim ako izričito ne razumeju uključeni alat.

The **`contextSnapshot`** beleži kontekst komentara, stranice i korisnika iz kojeg je zahtev za odobrenje stavljen u red. Njegov oblik odražava trigger-ovu kontekstualnu poruku.

### `approval.decided`

`data` šema:

[inline-code-attrs-start title = 'Šema podataka odluke o odobrenju'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - userId moderatora koji je doneo odluku",
  "decidedAt": "string - ISO 8601 - neobavezno, prisutno samo nakon odluke",
  "executedAt": "string - ISO 8601 - prisutno kada je APPROVED i izvršenje završeno",
  "executionResult": "string - poruka rezultata izvršitelja - prisutno nakon izvršenja",
  "contextSnapshot": { /* isto kao approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Šema TenantAgentAction'; type='json' inline-code-attrs-end]
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

`type` enum values match `AgentActionType`:

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

`SEARCH_MEMORY` se ne pojavljuje u `actions[]` zato što je samo za čitanje i nije auditan.

### `triggerType` enum values

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
- 15: `REPLAY` (internal; not delivered to webhooks)

### Zaglavlja

Svaka isporuka uključuje:

- `X-FastComments-Agent-Event` - kanoničko ime događaja (`trigger.succeeded`, itd.).
- `X-FastComments-Signature` - HMAC-SHA256 sirovog tela koristeći vašu API tajnu. Vidi [Potpisivanje Webhook-a](#webhook-signing).

### Stabilnost

Polja omotača i dokumentovana `data` polja po događaju su deo javnog ugovora. Dodavanje novih opcionih polja postojećim payload-ovima je dozvoljeno i ne smatra se prekidnom promenom - vaš potrošač treba da ignoriše nepoznata polja. Oblik `args` i `contextSnapshot` **nije** deo ugovora.

---