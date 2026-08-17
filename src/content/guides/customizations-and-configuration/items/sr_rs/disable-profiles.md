[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће приказати профил корисника када кликнете на њихов аватар.

Међутим, можемо онемогућити ову функцију:

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = 'Онемогући профиле'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте одељак „Онемогући профиле“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='Страница за прилагођавање виџета са означеним пољем за онемогућавање профила, тако да аватари више не отварају профиле'; title='Онемогући профиле' app-screenshot-end]