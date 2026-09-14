Štiri javne vals, ki jih lahko remiksate, vsaka pokriva en del tega vodnika.

**[Blog s komentarji](https://www.val.town/x/fastcomments/blog-with-comments)** ([živo](https://fastcomments-blog.val.run)) je Markdown blog z nitjo pod vsakim objavom in skupnimi števci komentarjev na indeksu. Deluje takoj, ko ga remiksate, in ena spremenljivka okolja ga usmeri na vaš račun.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([živo](https://fastcomments-sso.val.run)) prijavi obiskovalca s svojim računom Val Town in to identiteto predaja gradniku, tako da ni drugega prijavljanja.

**[Prejemnik Webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([živo](https://fastcomments-webhooks.val.run)) preveri HMAC podpis pri vsaki dostavi in shranjuje dogodke v SQLite. Ima gumb, ki podpiše testni paket podatkov in ga pošlje samemu sebi, tako da lahko opazujete uspešno preverjanje, preden konfigurirate pravi webhook.

**[Spretnosti agenta](https://www.val.town/x/fastcomments/skills)** ([živo](https://fastcomments-skills.val.run)) je knjižnica spretnosti agenta FastComments, ki pokrivajo gradnik, SSO, REST API, moderacijo in migracijo iz Disqus. Remiksajte jo in agent Val Town, Townie, samodejno prebere spretnosti iz `skills/`, tako da vaš agent ve, kako nastaviti komentarje, ne da bi morali lepljivo vstavljati dokumentacijo v klepet.

Enake spretnosti lahko namestite kjerkoli drugje z `npx skills add fastcomments/skills`.