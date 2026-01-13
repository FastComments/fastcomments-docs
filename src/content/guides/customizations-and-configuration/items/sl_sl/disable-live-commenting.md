[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Privzeto ima FastComments vklopljeno živo komentiranje.

To pomeni, da naj bi vsak gledalec niti komentarjev videl enako vsebino.

Na primer, če je komentar dodan, se mora ta komentar prikazati. Če je komentar urejen ali odstranjen,
bodo ti komentarji za vse gledalce niti prav tako urejeni ali odstranjeni. Enako velja za glasove in vse ukrepe moderiranja.

Vendar pa lahko to onemogočimo:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

To lahko storite tudi brez kode. Na strani za prilagajanje gradnika poglejte razdelek "Onemogoči živo komentiranje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---