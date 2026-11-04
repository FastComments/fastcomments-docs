Wszystkie ładunki webhooków agentów mają wspólną kopertę i dodają blok `data` specyficzny dla zdarzenia. Ta strona zawiera pełny schemat dla każdego z nich.

### Koperta (każde zdarzenie)

Każdy ładunek, niezależnie od typu zdarzenia, ma te pola najwyższego poziomu:

[inline-code-attrs-start title = 'Schemat koperty webhooka'; type='json' inline-code-attrs-end]
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

Schemat `data`:

[inline-code-attrs-start title = 'Schemat danych zdarzenia trigger'; type='json' inline-code-attrs-end]
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

`triggerType` to numeryczne wyliczenie z [listy zdarzeń trigger](#triggers-overview).

`actions[].type` to numeryczne wyliczenie z [listy narzędzi](#tools-overview).

`actions[].pending` ma wartość `true`, gdy akcja została zakolejkowana do [zatwierdzenia](#approval-workflow) zamiast zostać wykonana.

### `approval.requested`

Schemat `data`:

[inline-code-attrs-start title = 'Schemat danych approval.requested'; type='json' inline-code-attrs-end]
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

Obiekt **`args`** zawiera to, co przekazało wywołanie narzędzia LLM. Jego kształt zależy od narzędzia:

- Dla `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Dla `mark_comment_spam`: `{ commentId, isSpam }`.
- Dla `write_comment`: `{ comment, urlId, parentId? }`.
- ...i tak dalej.

Zbiór kształtów argumentów narzędzi **nie jest stabilnym kontraktem publicznym**. Narzędzia mogą być dodawane w przyszłości, a platforma przekazuje argumenty bez zmian. Konsumenci powinni traktować argumenty jako nieprzezroczysty blob, chyba że wprost rozumieją dane narzędzie.

**`contextSnapshot`** przechwytuje kontekst komentarza, strony i użytkownika, z którego zakolejkowano zatwierdzenie. Jego kształt odpowiada komunikatowi kontekstowemu triggera.

### `approval.decided`

Schemat `data`:

[inline-code-attrs-start title = 'Schemat danych approval.decided'; type='json' inline-code-attrs-end]
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

### Kształt TenantAgentAction

Wewnątrz `actions[]` w ładunkach triggerów każda akcja ma:

[inline-code-attrs-start title = 'Schemat TenantAgentAction'; type='json' inline-code-attrs-end]
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

Wartości wyliczenia `type` odpowiadają `AgentActionType`:

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

`SEARCH_MEMORY` nie pojawia się w `actions[]`, ponieważ jest tylko do odczytu i nie jest audytowane.

### Wartości wyliczenia `triggerType`

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
- 15: `REPLAY` (wewnętrzne; nie dostarczane do webhooków)

### Nagłówki

Każde dostarczenie zawiera:

- `X-FastComments-Agent-Event` - kanoniczna nazwa zdarzenia (`trigger.succeeded` itd.).
- `X-FastComments-Signature` - HMAC-SHA256 surowej treści z użyciem Twojego sekretu API. Zobacz [Podpisywanie webhooków](#webhook-signing).

### Stabilność

Pola koperty oraz udokumentowane pola `data` dla każdego zdarzenia są częścią kontraktu publicznego. Dodawanie nowych opcjonalnych pól do istniejących ładunków jest dozwolone i nie jest uważane za zmianę łamiącą - Twój konsument powinien ignorować nieznane pola. Kształt `args` i `contextSnapshot` **nie** jest częścią kontraktu.