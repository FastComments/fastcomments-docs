Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) jest blogiem w formacie Markdown z wątkiem pod każdym postem i zbiorczymi licznikami komentarzy na indeksie. Działa od razu po remixowaniu, a jedna zmienna środowiskowa wskazuje na twoje własne konto.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) loguje odwiedzającego przy użyciu jego konta Val Town i przekazuje tę tożsamość do widgetu, więc nie ma drugiego logowania.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) weryfikuje podpis HMAC przy każdej dostawie i przechowuje zdarzenia w SQLite. Ma przycisk, który podpisuje testowy payload i dostarcza go do siebie, więc możesz obserwować pomyślną weryfikację przed skonfigurowaniem prawdziwego webhooka.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) jest biblioteką umiejętności agenta FastComments obejmującą widget, SSO, REST API, moderację i migrację z Disqus. Remixuj ją, a agent Val Town, Townie, automatycznie pobiera umiejętności z `skills/`, więc twój agent wie, jak podłączyć komentarze bez wklejania dokumentacji do czatu.

The same skills install anywhere else with `npx skills add fastcomments/skills`.