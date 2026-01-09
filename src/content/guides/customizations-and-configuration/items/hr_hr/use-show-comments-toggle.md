[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazati polje za unos komentara i nit komentara istovremeno. Kako bi uštedio malo okomite površine,
također će sakriti sva ostala obavezna polja dok se widget ne upotrijebi.

Međutim, widget za komentare može biti skriven iza gumba, na primjer:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Gumb koristi različiti prevedeni tekst ovisno o tome prikazuju li se trenutni komentari ili ne. Ako su komentari skriveni, koristi `translations.SHOW_COMMENTS_BUTTON_TEXT`. Ako su
komentari prikazani, koristi `translations.HIDE_COMMENTS_BUTTON_TEXT`. Prijevodi mogu sadržavati tekst `[count]` koji će
biti zamijenjen lokaliziranim brojem.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Ovo je namijenjeno zamjenjivanju konfiguracije `hideCommentsUnderCountTextFormat`.

Broj se ažurira u stvarnom vremenu zajedno s niti komentara. Gumb se ne prikazuje ako nema komentara.

Ovo se može omogućiti bez koda stvaranjem pravila prilagodbe i omogućavanjem "Kliknite za prikaz komentara":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]