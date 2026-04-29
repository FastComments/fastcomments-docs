The **Community guidelines** field on the edit form is an optional policy text block included in the user-role context message on every run for this agent. It is fenced as untrusted text (the same fencing the platform applies to comment bodies and other user-supplied content), so the model treats it as policy reference, not as system instructions. It is the canonical place to write down "what behavior is and is not allowed on this site" so the agent applies it consistently.

### How it differs from the initial prompt

- **Initial prompt** - the agent's role and decision-making style. "You are a moderator. Prefer warning over banning."
- **Community guidelines** - the rules of your community, in policy language. "No personal attacks. No promotional links from accounts under 24 hours old. Off-topic comments may be removed if a thread is heated."

Both flow into the same context window, but they enter at different layers - the initial prompt is part of the system role, the guidelines doc is fenced text inside the user-role context message. The split makes editing easier when you want to update one without re-reading the other.

### What's a good guidelines doc

A short, specific, written-by-a-human document. Lists work better than prose:

[inline-code-attrs-start title = 'Community Guidelines Example'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

The agent applies this on every run. If you change the guidelines, the change takes effect on the next trigger - past runs are not retroactively re-evaluated.

### What not to put here

- **Output formatting instructions** ("reply in HTML", "use emoji"). Those belong in the [initial prompt](#personality-prompt).
- **Localized text.** The guidelines doc, like the prompt, is **English-only** for the same reason - machine translation can change agent behavior silently. If you have policies that vary by locale, write them all in English in this one document and structure the doc as "for German-language pages: ..."
- **Long quotations of external policies.** Paraphrase. Long context costs tokens on every run.
- **PII or secrets.** This text is sent to the LLM provider on every run.

### Length

The field is capped at **4000 characters** (enforced both by the form and the save route). Token cost on every run is proportional to length, so even within the cap a few hundred words is usually plenty. If your community policies run to many pages, summarize the parts the agent needs into a digest specifically for this field.

### Versioning

There is no built-in version history for the guidelines doc - the latest saved value is what the agent uses. If you want history, copy the doc into your own tracking system before each major edit. The [Refine Prompts](#refining-prompts) flow can record changes to the *initial prompt* but does not version the guidelines doc.
