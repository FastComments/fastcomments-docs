[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Када корисник први пут коментарише помоћу FastComments, покушаћемо да преузмемо његову аватару са <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Међутим, ако не пронађемо аватару, или корисник никада не постави једну у свом налогу, приказаћемо статичку подразумевану слику аватаре.

Да бисте навели сопствену статичку слику аватаре, можете користити подешавање *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Замените подразумевану аватару'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте одељак "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Одељак „Default Avatar“ на страници за прилагођавање виџета, где постављате URL резервне слике аватаре'; title='Прилагођавање подразумеване аватаре' app-screenshot-end]

Имајте на уму да је дефинисање аватаре за конкретног корисника, као што је случај са SSO, обрађено у посебном одељку.