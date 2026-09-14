Četiri javna val-a koja možete remixati, svako pokriva jedan dio ovog vodiča.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) je Markdown blog s nitima ispod svakog posta i grupiranim brojem komentara na indeksu. Radi odmah nakon što ga remixate, a jedna varijabla okruženja usmjerava ga na vaš vlastiti račun.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) prijavljuje posjetitelja s njihovim Val Town računom i predaje taj identitet widgetu, tako da nema drugog prijavljivanja.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) provjerava HMAC potpis na svakoj isporuci i pohranjuje događaje u SQLite. Ima gumb koji potpisuje testni payload i isporučuje ga samom sebi, tako da možete vidjeti da je provjera uspješna prije konfiguriranja pravog webhooka.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) je biblioteka FastComments agenata vještina koje pokrivaju widget, SSO, REST API, moderaciju i migraciju s Disqus‑a. Remixajte ga i Val Townov agent, Townie, automatski preuzima vještine iz `skills/`, tako da vaš agent zna kako postaviti komentare bez da vi lijepite dokumentaciju u chat.

Iste vještine instaliraju se bilo gdje drugdje pomoću `npx skills add fastcomments/skills`.