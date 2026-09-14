---  
Quatro vals públicos que você pode remixar, cada um cobrindo uma parte deste guia.

**[Blog com comentários](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) é um blog em Markdown com um tópico sob cada post e contagens de comentários em massa no índice. Ele funciona no momento em que você o remixar, e uma variável de ambiente aponta para sua própria conta.

**[Demo SSO](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) autentica o visitante com a conta do Val Town dele e entrega essa identidade ao widget, de modo que não há um segundo login.

**[Receptor de Webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifica a assinatura HMAC em cada entrega e armazena os eventos em SQLite. Ele possui um botão que assina uma carga de teste e a entrega a si mesmo, para que você possa observar a verificação ser bem‑sucedida antes de configurar um webhook real.

**[Habilidades de agente](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) é uma biblioteca de habilidades de agente FastComments que cobre o widget, SSO, a API REST, moderação e migração do Disqus. Remix‑a e o agente da Val Town, Townie, carrega as habilidades de `skills/` automaticamente, de modo que seu agente saiba como integrar comentários sem que você precise colar a documentação no chat.

As mesmas habilidades podem ser instaladas em qualquer outro lugar com `npx skills add fastcomments/skills`.