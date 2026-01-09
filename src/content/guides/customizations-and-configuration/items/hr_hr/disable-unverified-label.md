[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazati oznaku "Neprovjereni komentar" za komentare koji su ostavljeni korisniku koji ima nepotvrđenu sesiju preglednika. Više o neprovjerenom komentiranju pročitajte [ovdje](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Osim toga, ovu značajku možete koristiti, bez pisanja koda, u sučelju za prilagodbu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---