[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments će prikazati oznaku „Noverifikovan komentar“ za komentare koji su ostavljeni za korisnika koji ima neverifikovanu sesiju pretraživača. Pročitajte više o neverifikovanom komentarisanju [ovde](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Onemogući oznaku neverifikovanog'; code-example-end]

Pored toga, ova funkcija se može koristiti, bez pisanja koda, u UI‑u za prilagođavanje:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Stranica za prilagođavanje widgeta sa označenim poljem za onemogućavanje oznake neverifikovanog komentara'; title='Onemogući oznaku neverifikovanog' app-screenshot-end]