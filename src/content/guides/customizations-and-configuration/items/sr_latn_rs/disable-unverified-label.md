[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments prikazuje oznaku „Unverified Comment” za komentare koji su ostavljeni za korisnika koji ima neproverenu sesiju pretraživača. Pročitajte više o neproverenom komentarisanju [ovde](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Onemogući oznaku neproverenog komentara'; code-example-end]

Additionally, this feature can be used, without writing code, in the Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Stranica za prilagođavanje widgeta sa označenim poljem za onemogućavanje oznake neproverenog komentara'; title='Onemogući oznaku neproverenog komentara' app-screenshot-end]