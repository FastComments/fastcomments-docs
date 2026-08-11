[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će prikazati oznaku „Neprovjeren komentar” za komentare koji su ostavljeni za korisnika s neprovjerenom sesijom preglednika. Pročitajte više o neprovjerenom komentiranju [ovdje](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Onemogući oznaku neprovjerenog komentara'; code-example-end]

Dodatno, ovu značajku možete koristiti, bez pisanja koda, u sučelju za prilagodbu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Stranica za prilagodbu widgeta s označenim potvrdnim okvirom Onemogući oznaku neprovjerenog komentara'; title='Onemogući oznaku neprovjerenog komentara' app-screenshot-end]