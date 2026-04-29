Tüm agent webhook yükleri ortak bir zarf paylaşır ve olay-a özgü bir `data` bloğu ekler. Bu sayfa her biri için tam şemayı listeler.

### Zarf (her olay)

Olay türü ne olursa olsun, her payload şu üst düzey alanlara sahiptir:

[inline-code-attrs-start title = 'Webhook Zarf Şeması'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - bu teslimat için eşleşen alan adı",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 zaman damgası",
  "data": { /* olaya özgü, aşağıya bakın */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` şeması:

[inline-code-attrs-start title = 'Tetikleyici Olay Veri Şeması'; type='json' inline-code-attrs-end]
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
      "commentId": "string - isteğe bağlı",
      "userId": "string - isteğe bağlı",
      "badgeId": "string - isteğe bağlı",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - trigger.failed durumunda mevcut",
  "url": "string - isteğe bağlı",
  "urlId": "string - isteğe bağlı",
  "commentId": "string - isteğe bağlı"
}
[inline-code-end]

`triggerType`, [tetikleyici olay listesi](#triggers-overview) içinden sayısal bir enumdur.

`actions[].type`, [tool listesi](#tools-overview) içinden sayısal bir enumdur.

`actions[].pending`, eylem yürütülmek yerine [onay](#approval-workflow) için sıraya alınmışsa `true` olur.

### `approval.requested`

`data` şeması:

[inline-code-attrs-start title = 'Onay İstendi Veri Şeması'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* araca göre, aşağıya bakın */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - isteğe bağlı, ajan gerekçesi",
  "confidence": 0.85,
  "contextSnapshot": { /* onayın ilgili olduğu yorum/sayfa bağlamı */ }
}
[inline-code-end]

**`args`** nesnesi, LLM araç çağrısının taşıdığı her neyse odur. Şekli araçlara bağlıdır:

- `ban_user` için: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- `mark_comment_spam` için: `{ commentId, isSpam }`.
- `write_comment` için: `{ comment, urlId, parentId? }`.
- ...ve benzeri.

Araç argüman şekillerinin kümesi **kararlı bir genel sözleşme değildir**. Gelecekte araçlar eklenebilir ve platform args'i olduğu gibi iletir. Tüketiciler, ilgili aracı açıkça anlamadıkları sürece args'i opak bir blob olarak işlemelidir.

**`contextSnapshot`**, onayın sıraya alındığı yorumu, sayfayı ve kullanıcı bağlamını yakalar. Şekli, tetikleyicinin bağlam mesajını yansıtır.

### `approval.decided`

`data` şeması:

[inline-code-attrs-start title = 'Onay Kararı Veri Şeması'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - kararı veren moderatörün userId'si",
  "decidedAt": "string - ISO 8601 - isteğe bağlı, sadece karar verildikten sonra mevcut",
  "executedAt": "string - ISO 8601 - APPROVED olduğunda ve yürütme tamamlandığında mevcut",
  "executionResult": "string - yürütücü sonuç mesajı - yürütme sonrasında mevcut",
  "contextSnapshot": { /* approval.requested ile aynı */ }
}
[inline-code-end]

### TenantAgentAction şekli

Trigger payload'larındaki `actions[]` içinde, her eylemin yapısı:

[inline-code-attrs-start title = 'TenantAgentAction Şeması'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - isteğe bağlı",
  "userId": "string - isteğe bağlı",
  "badgeId": "string - isteğe bağlı",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` enum değerleri `AgentActionType` ile eşleşir:

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

`SEARCH_MEMORY`, salt okunur ve denetlenmeyen olduğu için `actions[]` içinde görünmez.

### `triggerType` enum değerleri

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
- 15: `REPLAY` (dahili; webhooks'lere gönderilmez)

### Başlıklar

Her teslimat şunları içerir:

- `X-FastComments-Agent-Event` - kanonik olay adı (`trigger.succeeded`, vb.).
- `X-FastComments-Signature` - ham gövdenin API sırrınızı kullanarak HMAC-SHA256'si. Bkz. [Webhook Signing](#webhook-signing).

### Kararlılık

Zarf alanları ve olay başına belgelenmiş `data` alanları genel sözleşmenin bir parçasıdır. Mevcut payload'lara yeni isteğe bağlı alanlar eklemek izinlidir ve kırıcı bir değişiklik olarak kabul edilmez - tüketiciniz bilinmeyen alanları yoksaymalıdır. `args` ve `contextSnapshot` şekli sözleşmenin bir parçası **değildir**.

---