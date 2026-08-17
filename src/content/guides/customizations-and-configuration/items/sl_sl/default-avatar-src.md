[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Ko uporabnik prvič komentira z FastComments, poskušamo pridobiti njegov avatar iz <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Vendar, če avatarja ne najdemo ali uporabnik nikoli ne nastavi avatarja v svoj račun, prikažemo statično privzeto sliko avatarja.

Za določitev lastne statične slike avatarja lahko uporabimo nastavitev *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Prepis privzetega avatarja'; code-example-end]

To lahko storite tudi brez kode. Na strani za prilagajanje gradnika si oglejte razdelek "Privzeti avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Razdelek Privzeti avatar na strani za prilagajanje gradnika, kjer nastavite nadomestni URL slike avatarja'; title='Prilagajanje privzetega avatarja' app-screenshot-end]

Upoštevajte, da je določanje avatarja za določenega uporabnika, na primer s SSO, obravnavano v svojem razdelku.