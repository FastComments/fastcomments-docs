[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Privzeto FastComments omogoča uporabnikom, da blokirajo druge uporabnike. Blokiranje uporabnika bo povzročilo, da se njihovi komentarji skrijejo, preprečuje obvestila med uporabniki in podobno.

Morda bo zaželeno onemogočiti to funkcionalnost. To lahko storite na naslednji način:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Onemogoči blokiranje'; code-example-end]

To je mogoče storiti tudi brez kode, kar omogoča tudi ustrezno strežniško validacijo, prek uporabniškega vmesnika za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Možnost onemogočanja blokiranja v uporabniškem vmesniku za prilagajanje gradnika, ki preprečuje uporabnikom, da bi se med seboj blokirali'; title='Onemogoči blokiranje' app-screenshot-end]