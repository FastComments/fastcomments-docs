A coding agent such as Claude Code, Cursor, or an MCP-based assistant can set up FastComments for you without you filling in the signup form. This is useful when you ask an agent to "add comments to my site" and you do not have an account yet.

### Як це працює

1. The agent creates a new account and receives an API key and a claim link. The API key works right away, so the agent can configure the account and install the widget on your site.
2. The agent gives you the claim link. Open it in your browser, log in or create a login, and confirm the claim. The account is then yours: you manage it, its billing, and its API keys from the dashboard. The page lists the API key the agent holds so you can revoke it if you no longer want the agent, or whoever runs it, to have access.
3. If nobody opens the claim link within 72 hours, the account and its key are deleted. Ask the agent to create a new one.

Until it is claimed, the account has the same limits as a normal free trial.

### Якщо у вас вже є обліковий запис

Each login owns one account. If you open a claim link while logged in to an existing account, the page lets you choose:

- **Attach to my account** makes the new account a managed tenant of the one you are logged in to. This needs a paid plan with white labeling, and the new tenant's usage is billed to your account.
- **Sign out and claim with another login** signs you out and brings you back to the claim page so you can claim it with a different login.

### Для авторів агентів

The agent instructions at [fastcomments.com/agents.md](https://fastcomments.com/agents.md) describe the account creation call, the fields in the response, and how to hand the claim link to the person you are working for. The call needs no API key and is rate limited per IP address.