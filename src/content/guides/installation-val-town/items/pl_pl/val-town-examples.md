Cztery publiczne valy do remiksu, z których każdy obejmuje jeden element tego przewodnika.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) to blog w Markdown z wątkiem komentarzy pod każdym wpisem i zbiorczymi licznikami komentarzy na stronie głównej. Działa od razu po remiksie, a jedna zmienna środowiskowa wskazuje go na Twoje własne konto.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) loguje odwiedzającego za pomocą konta Val Town i przekazuje tę tożsamość do widgetu, więc nie ma drugiego logowania.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) weryfikuje podpis HMAC przy każdej dostawie i zapisuje zdarzenia w SQLite. Ma przycisk, który podpisuje testowy payload i dostarcza go do samego siebie, więc możesz zobaczyć, jak weryfikacja działa, zanim skonfigurujesz prawdziwy webhook.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) to biblioteka umiejętności agenta FastComments obejmująca widget, SSO, REST API, moderację i migrację z Disqus. Zremiksuj ją, a agent Val Town, Townie, automatycznie pobierze umiejętności z `skills/`, więc Twój agent będzie wiedział, jak podłączyć komentarze, bez wklejania dokumentacji do czatu.

Te same umiejętności można zainstalować gdziekolwiek indziej za pomocą `npx skills add fastcomments/skills`.