[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Po defaultu, FastComments će prikazati karticu "Direktne poruke" na profilima korisnika, omogućavajući posetiocima da pošalju direktne poruke korisniku.

Međutim, možemo onemogućiti ovu karticu:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak "Onemogući direktne poruke".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]