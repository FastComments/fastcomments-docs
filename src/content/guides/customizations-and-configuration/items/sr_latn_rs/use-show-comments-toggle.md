[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments će prikazati polje za unos komentara i nit komentara istovremeno. Da bi uštedeli vertikalni prostor,
takođe će sakriti sve druge neophodne polja dok se widget ne interaguje.

Međutim, widget za komentare može biti sakriven iza dugmeta, na primer:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Widget za komentare skupljen iza dugmeta koje prikazuje broj komentara dok čitač ne klikne na njega'; title='Klikni da prikažeš komentare' app-screenshot-end]

Dugme koristi različit prevedeni tekst u zavisnosti da li su komentari trenutno prikazani ili ne. Ako su komentari sakriveni, koristi `translations.SHOW_COMMENTS_BUTTON_TEXT`. Ako su
komentari prikazani, koristi `translations.HIDE_COMMENTS_BUTTON_TEXT`. Prevođenja mogu sadržati tekst `[count]` koji će
biti zamenjen lokalizovanim brojem.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Klikni da prikažeš ili sakriješ komentare'; code-example-end]

Ovo je dizajnirano da zameni konfiguraciju `hideCommentsUnderCountTextFormat`.

Broj se ažurira u realnom vremenu zajedno sa nitom komentara. Dugme se ne prikazuje ako nema komentara.

Ovo se može omogućiti bez koda kreiranjem pravila prilagođavanja i omogućavanjem "Klikni da prikažeš komentare":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Polje za potvrdu Klikni da prikažeš komentare označeno u pravilu prilagođavanja na stranici za prilagođavanje widgeta'; title='Omogući Klikni da prikažeš komentare' app-screenshot-end]