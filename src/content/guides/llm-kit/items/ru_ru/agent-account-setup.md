A coding agent such as Claude Code, Cursor, or an MCP-based assistant can set up FastComments for you without you filling in the signup form. This is useful when you ask an agent to "add comments to my site" and you do not have an account yet.

### Как это работает

1. The agent creates a new account and receives an API key and a claim link. The API key works right away, so the agent can configure the account and install the widget on your site.  
   **Агент создает новый аккаунт и получает API‑ключ и ссылку для подтверждения. API‑ключ работает сразу, поэтому агент может настроить аккаунт и установить виджет на ваш сайт.**

2. The agent gives you the claim link. Open it in your browser, log in or create a login, and confirm the claim. The account is then yours: you manage it, its billing, and its API keys from the dashboard. The page lists the API key the agent holds so you can revoke it if you no longer want the agent, or whoever runs it, to have access.  
   **Агент дает вам ссылку для подтверждения. Откройте её в браузере, войдите в систему или создайте учетную запись, и подтвердите запрос. После этого аккаунт будет вашим: вы управляете им, его биллингом и API‑ключами через панель управления. На странице отображается API‑ключ, которым владеет агент, чтобы вы могли отозвать его, если больше не хотите, чтобы агент или кто‑то, кто его использует, имел доступ.**

3. If nobody opens the claim link within 72 hours, the account and its key are deleted. Ask the agent to create a new one.  
   **Если никто не откроет ссылку для подтверждения в течение 72 часов, аккаунт и его ключ будут удалены. Попросите агента создать новый.**

Until it is claimed, the account has the same limits as a normal free trial.  
**Пока аккаунт не подтвержден, он имеет те же ограничения, что и обычный бесплатный пробный период.**

### Если у вас уже есть аккаунт

Each login owns one account. If you open a claim link while logged in to an existing account, the page lets you choose:  
**Каждый вход (логин) владеет одним аккаунтом. Если вы откроете ссылку для подтверждения, будучи вошедшими в существующий аккаунт, страница предложит вам выбрать:**

- **Attach to my account** makes the new account a managed tenant of the one you are logged in to. This needs a paid plan with white labeling, and the new tenant's usage is billed to your account.  
  **Attach to my account** делает новый аккаунт управляемым арендатором того, в который вы вошли. Для этого требуется платный план с белой маркировкой, и использование нового арендатора будет списываться с вашего аккаунта.

- **Sign out and claim with another login** signs you out and brings you back to the claim page so you can claim it with a different login.  
  **Sign out and claim with another login** выводит вас из системы и возвращает на страницу подтверждения, чтобы вы могли подтвердить её с другим логином.

### Для авторов агентов

The agent instructions at [fastcomments.com/agents.md](https://fastcomments.com/agents.md) describe the account creation call, the fields in the response, and how to hand the claim link to the person you are working for. The call needs no API key and is rate limited per IP address.  
**Инструкции для агентов на странице [fastcomments.com/agents.md](https://fastcomments.com/agents.md) описывают запрос создания аккаунта, поля в ответе и то, как передать ссылку для подтверждения человеку, для которого вы работаете. Запрос не требует API‑ключа и ограничен по частоте запросов для каждого IP‑адреса.**