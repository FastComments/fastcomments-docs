[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказивати опције гласања као стрелице горе и доле, омогућавајући корисницима да гласају за или против коментара.

Међутим, могуће је променити стил траке за гласање. Тренутне опције су подразумевани тастери Горе/Доле, или коришћење механизма гласања у облику срца.

Користимо флаг **voteStyle** на следећи начин:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Снажно препоручујемо да ово урадите без кода јер такође омогућава валидацију на серверској страни. На страници за прилагођавање видгета, погледајте одељак "Стил гласања".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Гласање такође може бити онемогућено, погледајте `Disable Voting` изнад опција стила.

---