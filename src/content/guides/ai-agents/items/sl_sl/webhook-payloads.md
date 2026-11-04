Vsi spletni webhook payloadi agentov imajo skupno ovojnico in dodajo blok `data`, specifičen za dogodek. Ta stran navaja polno shemo za vsakega.

### Ovojnica (vsak dogodek)

Vsak payload, ne glede na vrsto dogodka, ima ta polja na vrhu:

[inline-code-attrs-start title = 'Shema ovojnice webhooka'; type='json' inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Shema podatkov dogodka trigger'; type='json' inline-code-attrs-end]
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

`triggerType` je številčni enum iz [seznama dogodkov trigger](#triggers-overview).

`actions[].type` je številčni enum iz [seznama orodij](#tools-overview).

`actions[].pending` je `true`, ko je bilo dejanje namesto izvedeno postavljeno v čakalno vrsto za [odobritev](#approval-workflow).

### `approval.requested`

Shema `data`:

[inline-code-attrs-start title = 'Shema podatkov approval.requested'; type='json' inline-code-attrs-end]
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

Objekt **`args`** je tisto, kar je prinesel klic orodja LLM. Njegova oblika je odvisna od orodja:

- Za `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Za `mark_comment_spam`: `{ commentId, isSpam }`.
- Za `write_comment`: `{ comment, urlId, parentId? }`.
- ...in tako naprej.

Nabor oblik argumentov orodij **ni stabilna javna pogodba**. Orodja se lahko v prihodnje dodajajo, platforma pa argumente prenaša nespremenjene. Potrošniki naj argumente obravnavajo kot neprozorno blob, razen če izrecno razumejo vpleteno orodje.

**`contextSnapshot`** zajema kontekst komentarja, strani in uporabnika, iz katerega je bila odobritev postavljena v čakalno vrsto. Njegova oblika odraža kontekstno sporočilo triggerja.

### `approval.decided`

Shema `data`:

[inline-code-attrs-start title = 'Shema podatkov approval.decided'; type='json' inline-code-attrs-end]
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

### Oblika TenantAgentAction

Znotraj `actions[]` na payloadih trigger ima vsako dejanje:

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

Vrednosti enuma `type` ustrezajo `AgentActionType`:

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

`SEARCH_MEMORY` se ne pojavlja v `actions[]`, ker je samo za branje in ni reviziran.

### Vrednosti enuma `triggerType`

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
- 15: `REPLAY` (interno; ni dostavljeno na webhooke)

### Glave

Vsaka dostava vključuje:

- `X-FastComments-Agent-Event` - kanonično ime dogodka (`trigger.succeeded` itd.).
- `X-FastComments-Signature` - HMAC-SHA256 surovega telesa z vašim API skrivnim ključem. Glejte [Podpisovanje webhookov](#webhook-signing).

### Stabilnost

Polja ovojnice in dokumentirana polja `data` za vsak dogodek so del javne pogodbe. Dodajanje novih neobveznih polj obstoječim payloadom je dovoljeno in se ne šteje za spremembo, ki bi povzročila nezdružljivost - vaš potrošnik naj prezre neznana polja. Oblika `args` in `contextSnapshot` **ni** del pogodbe.