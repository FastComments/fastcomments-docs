[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Када корисник први пут коментарише помоћу FastComments-а, покушаћемо да преузмемо њихов аватар са <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Међутим, ако не пронађемо аватар или корисник никада не постави један у свом налогу, приказујемо статичну подразумевану слику аватара.

Да бисте навели своју статичну слику аватара, можете користити подешавање *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Замена подразумеваног аватара'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање видгета, погледајте одељак "Подразумевани аватар".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Прилагођавање подразумеваног аватара' app-screenshot-end]

Имајте на уму да је дефинисање аватара за појединачног корисника, као нпр. уз SSO, обрађено у свом засебном одељку.