[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments prikazuje karticu „Direktne poruke“ na korisničkim profilima, omogućavajući posetiocima da pošalju direktne poruke korisniku.

Međutim, možemo onemogućiti ovu karticu:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Onemogući direktne poruke na profilu'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Onemogući direktne poruke“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Stranica za prilagođavanje widgeta sa označenim poljem za onemogućavanje direktnih poruka, čime se sakriva kartica poruka na profilu'; title='Onemogući direktne poruke na profilu' app-screenshot-end]