[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će prikazivati oznaku "Neverifikovan komentar" za komentare koji su ostavljeni za korisnika koji
ima neverifikovanu sesiju u pregledaču. Pročitajte više o neverifikovanom komentisanju [ovde](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Pored toga, ova funkcija se može koristiti, bez pisanja koda, u korisničkom interfejsu za prilagođavanje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---