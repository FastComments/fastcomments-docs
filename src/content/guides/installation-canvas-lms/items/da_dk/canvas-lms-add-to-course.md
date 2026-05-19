#### Hvordan kommentarer vises i dine kurser

Når LTI-integrationen er aktiveret og External App er installeret, fungerer FastComments automatisk baseret på de placeringer, du har konfigureret:

#### Assignment View

Hvis placeringen **Assignment View** er aktiveret, vises kommentarer automatisk under hver opgave i kurset. Studerende og undervisere ser en trådet kommentarsektion, når de åbner en opgave — der kræves ingen ekstra opsætning per opgave.

Hver opgave får sin egen separate kommentartråd.

#### Rich Content Editor Button

Hvis placeringen **Editor Button** er aktiveret, kan undervisere indlejre FastComments i ethvert indhold, der bruger Rich Content Editor:

1. Rediger en **Page**, **Quiz**, eller **Announcement**.
2. I værktøjslinjen i Rich Content Editor skal du klikke på **FastComments**-knappen.
3. FastComments indlejres automatisk i indholdet.
4. Gem siden.

Når studerende åbner siden, indlæses den indlejrede FastComments-widget med en kommentartråd, der er unik for den pågældende side.

#### Automatic SSO

I begge placeringer bliver studerende automatisk logget ind via deres Canvas-konto. Navne, e-mails og avatarer synkroniseres gennem LTI-launch, så der ikke er behov for en separat login.

#### Lock Down Public Access (Recommended)

Som standard er FastComments-kommentardata offentligt læsbare. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se kommentarerne, selv uden for Canvas. Til kursusdiskussioner vil du næsten altid ønske at begrænse visningen til kun tilmeldte studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-launch.

Se [Beskyt kommentartråde med Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, inklusive hvordan du afgrænser reglen til et enkelt domæne eller en enkelt side.