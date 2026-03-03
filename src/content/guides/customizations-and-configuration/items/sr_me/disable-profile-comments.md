[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments će prikazati karticu "Komentari profila" na korisničkim profilima, omogućavajući posjetiocima da ostave komentare na nečijem profilu.

Međutim, možemo onemogućiti ovu karticu:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Onemogući komentare na profilu'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte odjeljak "Onemogući komentare na profilu".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Onemogući komentare na profilu' app-screenshot-end]