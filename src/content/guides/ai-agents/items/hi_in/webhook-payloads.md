सभी एजेंट वेबहुक पेलोड्स एक सामान्य एनवेलप साझा करते हैं और एक इवेंट-विशिष्ट `data` ब्लॉक जोड़ते हैं। यह पेज प्रत्येक के लिए पूरी स्कीमा सूचीबद्ध करता है।

### एनवेलप (हर इवेंट)

हर पेलोड, इवेंट प्रकार की परवाह किए बिना, इन टॉप-लेवल फ़ील्ड्स को रखता है:

[inline-code-attrs-start title = 'वेबहुक एनवेलप स्कीमा'; type='json' inline-code-attrs-end]
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

`data` स्कीमा:

[inline-code-attrs-start title = 'ट्रिगर इवेंट डेटा स्कीमा'; type='json' inline-code-attrs-end]
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

`triggerType` [ट्रिगर इवेंट सूची](#triggers-overview) से एक संख्यात्मक एनम है।

`actions[].type` [टूल सूची](#tools-overview) से एक संख्यात्मक एनम है।

`actions[].pending` `true` होता है जब एक्शन को निष्पादित करने के बजाय [अनुमोदन](#approval-workflow) के लिए कतार में रखा गया था।

### `approval.requested`

`data` स्कीमा:

[inline-code-attrs-start title = 'अनुमोदन अनुरोधित डेटा स्कीमा'; type='json' inline-code-attrs-end]
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

**`args`** ऑब्जेक्ट वह है जो LLM टूल कॉल ने ले जाया था। इसका आकार टूल पर निर्भर करता है:

- `ban_user` के लिए: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`।
- `mark_comment_spam` के लिए: `{ commentId, isSpam }`।
- `write_comment` के लिए: `{ comment, urlId, parentId? }`।
- ...और इसी तरह।

टूल आर्ग्युमेंट आकारों का सेट **एक स्थिर सार्वजनिक अनुबंध नहीं है**। भविष्य में टूल जोड़े जा सकते हैं और प्लेटफ़ॉर्म args को ज्यों का त्यों पास करता है। उपभोक्ताओं को args को एक अपारदर्शी ब्लॉब मानना चाहिए जब तक कि वे स्पष्ट रूप से शामिल टूल को न समझते हों।

**`contextSnapshot`** कमेंट, पेज और यूज़र संदर्भ को कैप्चर करता है जिससे अनुमोदन कतार में रखा गया था। इसका आकार ट्रिगर के संदर्भ संदेश को दर्शाता है।

### `approval.decided`

`data` स्कीमा:

[inline-code-attrs-start title = 'अनुमोदन निर्णय डेटा स्कीमा'; type='json' inline-code-attrs-end]
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

### TenantAgentAction आकार

ट्रिगर पेलोड्स पर `actions[]` के अंदर, प्रत्येक एक्शन में होता है:

[inline-code-attrs-start title = 'TenantAgentAction स्कीमा'; type='json' inline-code-attrs-end]
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

`type` एनम मान `AgentActionType` से मेल खाते हैं:

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

`SEARCH_MEMORY` `actions[]` में नहीं दिखता क्योंकि यह केवल-पढ़ने योग्य और अनॉडिटेड है।

### `triggerType` एनम मान

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
- 15: `REPLAY` (आंतरिक; वेबहुक्स को डिलीवर नहीं किया जाता)

### हेडर्स

हर डिलीवरी में शामिल है:

- `X-FastComments-Agent-Event` - कैनोनिकल इवेंट नाम (`trigger.succeeded`, आदि)।
- `X-FastComments-Signature` - आपके API सीक्रेट का उपयोग करके रॉ बॉडी का HMAC-SHA256। [वेबहुक साइनिंग](#webhook-signing) देखें।

### स्थिरता

एनवेलप फ़ील्ड्स और प्रति इवेंट प्रलेखित `data` फ़ील्ड्स सार्वजनिक अनुबंध का हिस्सा हैं। मौजूदा पेलोड्स में नए वैकल्पिक फ़ील्ड्स जोड़ना अनुमत है और इसे ब्रेकिंग बदलाव नहीं माना जाता - आपके उपभोक्ता को अज्ञात फ़ील्ड्स को अनदेखा करना चाहिए। `args` और `contextSnapshot` का आकार अनुबंध का हिस्सा **नहीं** है।