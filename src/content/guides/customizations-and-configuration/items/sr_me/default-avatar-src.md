---
[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Када корисник први пут коментарише преко FastComments, покушаћемо да преузмемо њихов аватар са <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Међутим, ако не пронађемо аватар, или корисник никада не подеси аватар у свом налогу, приказујемо статичку подразумевану слику аватара.

Да бисте навели своју статичку слику аватара, можете користити поставку *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте одељак "Подразумевани аватар".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Имајте на уму да је дефинисање аватара за одређеног корисника, као што је у случају SSO, обрађено у посебном одељку.

---