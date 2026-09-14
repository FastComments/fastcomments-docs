Vier öffentliche Vals, die Sie remixen können, jede deckt einen Teil dieses Leitfadens ab.

**[Blog mit Kommentaren](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) ist ein Markdown‑Blog mit einem Thread unter jedem Beitrag und einer Gesamtsumme der Kommentare im Index. Er funktioniert sofort, wenn Sie ihn remixen, und eine Umgebungsvariable verweist auf Ihr eigenes Konto.

**[SSO‑Demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) meldet den Besucher mit seinem Val‑Town‑Konto an und übergibt diese Identität an das Widget, sodass kein zweiter Login nötig ist.

**[Webhook‑Empfänger](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) prüft die HMAC‑Signatur bei jeder Zustellung und speichert Ereignisse in SQLite. Er hat einen Button, der eine Test‑Payload signiert und an sich selbst liefert, sodass Sie die erfolgreiche Verifizierung beobachten können, bevor Sie einen echten Webhook konfigurieren.

**[Agent‑Skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) ist eine Bibliothek von FastComments‑Agent‑Skills, die das Widget, SSO, die REST‑API, Moderation und die Migration von Disqus abdecken. Remixen Sie sie und der Agent von Val Town, Townie, übernimmt die Skills automatisch aus `skills/`, sodass Ihr Agent weiß, wie Kommentare eingerichtet werden, ohne dass Sie Dokumentation in den Chat einfügen müssen.

Die gleichen Skills können überall sonst mit `npx skills add fastcomments/skills` installiert werden.