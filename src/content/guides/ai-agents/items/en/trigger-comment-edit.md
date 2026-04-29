Fires the agent when a comment is edited.

### Context the agent receives

- The comment in its current (post-edit) form.
- The **previous comment text** as a separate fenced block (`PREVIOUS_TEXT`). This is unique to the edit trigger - the agent can compare before/after.
- Optional thread / user history / page context as configured.

### Notable

- The trigger fires for any successful edit, including edits performed by moderators on behalf of a user.
- Agents have no edit-comment tool exposed to them; agents cannot edit comments at all.
- The previous comment text is fenced as untrusted input. The platform's system prompt reminds the model not to follow instructions from inside fences - this matters here, because a malicious user could edit a comment to insert a "ignore your previous instructions" payload aimed at any agent watching edit events.

### Common uses

- **Catching laundered content** - a user edits a previously-clean comment to insert spam after the moderator has moved on.
- **Tracking minor edits** - if your community treats edits as separate events for any audit reason.

### Cost note

Edit triggers see two copies of the comment text (the new version in the standard COMMENT block, the old version in the PREVIOUS_TEXT block). For long comments this roughly doubles the token cost of the run vs. a `COMMENT_ADD` trigger - keep that in mind when budgeting.
