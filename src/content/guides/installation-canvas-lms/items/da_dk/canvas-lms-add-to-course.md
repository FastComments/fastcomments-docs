#### Sådan vises kommentarer i dine kurser

Når LTI-integrationen er aktiveret, og External App er installeret, fungerer FastComments automatisk baseret på de placeringer, du har konfigureret:

#### Assignment View

Hvis placeringen **Assignment View** er aktiveret, vises kommentarer automatisk under hver opgave i kurset. Studerende og undervisere ser en trådet kommentarsektion, når de ser en opgave — der kræves ingen ekstra opsætning pr. opgave.

Hver opgave får sin egen separate kommentartråd.

#### Rich Content Editor Button

Hvis placeringen **Editor Button** er aktiveret, kan undervisere indlejre FastComments i alt indhold, der bruger Rich Content Editor:

1. Rediger en **Page**, **Quiz** eller **Announcement**.
2. Klik på **FastComments**-knappen i værktøjslinjen i Rich Content Editor.
3. FastComments indlejres automatisk i indholdet.
4. Gem siden.

Når studerende ser siden, indlæses den indlejrede FastComments-widget med en kommentartråd, der er unik for den pågældende side.

#### Automatisk SSO

I begge placeringer logges de studerende automatisk ind via deres Canvas-konto. Navne, e-mails og avatars synkroniseres gennem LTI-launch, der kræves ingen separat login.

#### Lås offentlig adgang (anbefales)

Som standard er FastComments-kommentardata offentligt læsbare. Alle, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, selv uden for Canvas. Til kursusdiskussioner vil du næsten helt sikkert begrænse visningen til kun tilmeldte studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-tilpasningsside</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-launch.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, herunder hvordan du afgrænser reglen til et enkelt domæne eller en enkelt side.