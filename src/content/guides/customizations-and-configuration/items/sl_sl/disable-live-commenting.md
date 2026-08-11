[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments imel omogočeno živo komentiranje.

To pomeni, da naj bi vsak gledalec niti komentarjev videl enako vsebino.

Na primer, če je komentar dodan, se naj ta komentar prikaže. Če je komentar urejen ali odstranjen,
potem bodo ti komentarji urejeni ali odstranjeni za vse gledalce niti. Enako velja za glasove in vse moderacijske akcije.

Vendar pa lahko to onemogočimo:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

To je mogoče storiti tudi brez kode. Na strani za prilagajanje gradnika poiščite razdelek "Onemogoči živo komentiranje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Razdelek Onemogoči živo komentiranje na strani za prilagajanje gradnika, ki izklopi posodobitve niti v realnem času'; title='Onemogoči živo komentiranje' app-screenshot-end]