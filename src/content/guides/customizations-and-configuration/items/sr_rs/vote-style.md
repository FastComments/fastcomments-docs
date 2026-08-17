[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Подразумевано, FastComments ће приказати опције гласања као стрелице за горе и доле, омогућавајући корисницима да гласују горе или доле на коментар.

Међутим, могуће је променити стил траке за гласање. Тренутне опције су подразумевана дугмад за горе/доле, или коришћење механизма гласања у облику срца.

Користимо заставицу **voteStyle** на следећи начин:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Omogući dugme srca'; code-example-end]

Топло вам препоручујемо да ово урадите без кода, јер тако такође омогућавате серверску валидацију. На страници за прилагођавање виџета, погледајте одељак „Vote Style“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Подешавање стила гласања на страници за прилагођавање виџета, нуди стрелице за горе и доле или гласовање срцем'; title='Промени стил гласања' app-screenshot-end]

Гласање се такође може онемогућити, погледајте `Disable Voting` изнад опција стила.