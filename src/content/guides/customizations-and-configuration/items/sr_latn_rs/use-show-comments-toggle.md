[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Po podrazumevanju, FastComments će istovremeno prikazati okvir za unos komentara i nit komentara. Da bi uštedeo vertikalni prostor,
takođe će sakriti sva ostala obavezna polja dok se ne interaguje sa vidžetom.

Međutim, vidžet za komentare može biti sakriven iza dugmeta, na primer:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Dugme koristi različit prevedeni tekst u zavisnosti od toga da li su komentari trenutno prikazani ili ne. Ako su komentari sakriveni, koristi `translations.SHOW_COMMENTS_BUTTON_TEXT`. Ako su komentari prikazani, koristi `translations.HIDE_COMMENTS_BUTTON_TEXT`. Prevodi mogu sadržati tekst `[count]` koji će
biti zamenjen lokalizovanim brojem.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Ovo je namenjeno da zameni konfiguraciju `hideCommentsUnderCountTextFormat`.

Brojač se ažurira u realnom vremenu zajedno sa niti komentara. Dugme se ne prikazuje ako nema komentara.

Ovo se može omogućiti bez koda kreiranjem pravila za prilagođavanje i omogućavanjem "Klikni za prikaz komentara":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]