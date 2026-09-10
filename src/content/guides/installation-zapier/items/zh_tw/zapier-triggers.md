## Triggers

Triggers start a Zap when something happens in FastComments. All three are instant: FastComments delivers the
event to Zapier through a webhook the moment it happens. Nothing polls your account and no API credits are
spent waiting.

| 觸發器 | 觸發時機 |
|---------|-----------|
| New Comment | 發表了一則評論。預設情況下，僅已批准且非垃圾評論會觸發。 |
| Updated Comment | 評論被編輯、批准、投票、置頂、鎖定或以其他方式變更。 |
| Deleted Comment | 評論被刪除。 |

Each trigger returns the full comment: id, page URL and URL ID, commenter name and email, the comment text
as markdown and as HTML, vote counts, approval and spam flags, the locale, the domain, and any mentions. The
fields match the webhook payload documented under Webhooks, Data Structures.

## Options

**Domain.** Every trigger has an optional domain filter, listing the domains configured on your account.
Leave it blank to receive events from every domain.

**Include Unapproved and Spam Comments.** On the New Comment trigger only. Comments that are held for
moderation or marked as spam are skipped by default. When such a comment is approved later, the Updated
Comment trigger fires for it, so a Zap that should react to every comment that becomes visible uses
Updated Comment with a filter on the approved field.

## How delivery works

Turning a Zap on creates a webhook subscription on your account, visible on the Webhooks page with the
source **API**. Turning the Zap off removes it. Zapier's own limits apply to how many events it accepts per
minute; FastComments retries a delivery that fails, with a growing delay, and disables a subscription that
keeps failing for six days. A disabled subscription can be re-enabled from the Webhooks page, or simply turn
the Zap off and on again to create a fresh one.

An account can hold up to 50 API subscriptions. Each Zap using a FastComments trigger uses one.

---