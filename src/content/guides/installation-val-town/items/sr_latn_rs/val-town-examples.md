Četiri javna vala koja možete remixovati, svako pokriva jedan deo ovog vodiča.

**[Blog sa komentarima](https://www.val.town/x/fastcomments/blog-with-comments)** ([uživo](https://fastcomments-blog.val.run)) je Markdown blog sa nitima ispod svakog posta i grupnim brojem komentara na indeksu. Radi odmah kada ga remixujete, a jedna promenljiva okruženja usmerava ga na vaš sopstveni nalog.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([uživo](https://fastcomments-sso.val.run)) prijavljuje posetioca pomoću njihovog Val Town naloga i predaje taj identitet widgetu, tako da nema drugog prijavljivanja.

**[Webhook prijemnik](https://www.val.town/x/fastcomments/webhook-receiver)** ([uživo](https://fastcomments-webhooks.val.run)) verifikuje HMAC potpis na svakoj isporuci i čuva događaje u SQLite. Ima dugme koje potpisuje testni payload i šalje ga samom sebi, tako da možete videti da verifikacija uspe pre podešavanja pravog webhook‑a.

**[Veštine agenta](https://www.val.town/x/fastcomments/skills)** ([uživo](https://fastcomments-skills.val.run)) je biblioteka FastComments veština agenta koja pokriva widget, SSO, REST API, moderaciju i migraciju sa Disqus‑a. Remixujte je i agent Val Town‑a, Townie, automatski preuzima veštine iz `skills/`, tako da vaš agent zna kako da postavi komentare bez da vi lepite dokumentaciju u ćaskanje.

Iste veštine se instaliraju bilo gde drugde pomoću `npx skills add fastcomments/skills`.