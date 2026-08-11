[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments prikazuje karticu „Komentari na profilu“ na korisničkim profilima, omogućavajući posetiocima da ostave komentare na nečijem profilu.

Međutim, možemo onemogućiti ovu karticu:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Onemogući komentare na profilu'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Onemogući komentare na profilu“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='Stranica za prilagođavanje widgeta sa označenim poljem Onemogući komentare na profilu da sakrije karticu komentara na profilu'; title='Onemogući komentare na profilu' app-screenshot-end]