Fires when a comment is auto-flagged as spam by FastComments' built-in spam engine - **not** by a moderator and not by another agent.

### Context the agent receives

- The auto-spammed comment.
- Optional thread / user history / page context as configured.

### Who fires this

The platform's spam pipeline. See [Spam Detection](/guide-moderation.html#spam-detection) in the moderation guide for more details.

### Common uses

- **Second-look moderation** - the spam engine has high recall but imperfect precision; an agent trained on your specific community style can catch false positives. The agent can call to un-flag a wrongly-classified comment.
- **Automated unbanning** - if your tenant aggressively spam-bans new accounts, an agent on this trigger can review and clear obvious false positives before a human ever sees them.

### Notable

- The trigger does **not** fire for moderator-marked spam (use [Trigger: Moderator Marked Spam](#trigger-moderator-spammed)) nor for spam marked by another agent.
- A comment that is auto-spammed and then later marked Not Spam by a moderator does not refire the trigger.
- Subscribing to this trigger is most useful in tenants where the engine's auto-spam mode is enabled under Moderation Settings. Otherwise the trigger will not fire.
