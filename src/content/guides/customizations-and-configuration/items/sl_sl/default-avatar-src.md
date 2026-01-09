[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Ko uporabnik prvič komentira z FastComments, bomo poskušali pridobiti njihov avatar z <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Če pa ne najdemo avatarja, ali uporabnik v svojem računu nikoli ne nastavi nobenega, prikažemo statično privzeto sliko avatarja.

Za določitev lastne statične slike avatarja lahko uporabimo nastavitev *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

To je mogoče narediti tudi brez kode. Na strani za prilagoditev vtičnika si oglejte razdelek "Privzeti avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Upoštevajte, da je določanje avatarja za posameznega uporabnika, na primer pri SSO, zajeto v svojem razdelku.