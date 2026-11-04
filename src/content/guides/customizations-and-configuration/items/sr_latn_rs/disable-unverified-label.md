[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će prikazati oznaku "Unverified Comment" za komentare koje je ostavio korisnik čija sesija pregledača nije verifikovana. Pročitajte više o komentisanju bez verifikacije [ovde](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Onemogući neverifikovanu oznaku'; code-example-end]

Dodatno, ova funkcija se može koristiti, bez pisanja koda, u korisničkom interfejsu za prilagođavanje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Stranica za prilagođavanje vidžeta sa čekiranim poljem Disable Unverified Comment Label'; title='Onemogući neverifikovanu oznaku' app-screenshot-end]