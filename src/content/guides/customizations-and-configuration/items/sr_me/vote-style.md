[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

По подразумеваном подешавању, FastComments ће приказати опције гласања као стрелице горе и доле, омогућавајући корисницима да гласају за или против коментара.

Међутим, могуће је промијенити стил траке за гласање. Тренутне опције су подразумевани тастери Горе/Доле или коришћење механизма гласања у облику срца.

Користимо ознаку **voteStyle** на следећи начин:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Снажно препоручујемо да ово урадите без кода јер то такође омогућава валидацију на серверској страни. На страници за прилагођавање виџета погледајте одељак „Стил гласања“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Гласање се такође може онемогућити, погледајте `Disable Voting` изнад опција за стил.

---