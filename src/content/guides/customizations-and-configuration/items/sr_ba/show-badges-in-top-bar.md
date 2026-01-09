[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

По подразумеву, FastComments ће приказивати значке корисника само на њиховим коментарима у нити коментара.

Међутим, можемо приказати значке корисника поред њиховог имена изнад форме за коментар ако омогућимо ову функцију на страници за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Ово ће приказати значке корисника поред њиховог имена у горњем делу траке, чинећи њихова достигнућа и статус видљивијим док пишу коментар.

Имајте на уму да ова функција мора бити омогућена у корисничком интерфејсу за прилагођавање видгета да би функционисала. Опционално можете поставити флаг **showBadgesInTopBar** на false у вашој конфигурацији кода како бисте га селективно онемогућили чак и када је укључен на нивоу сервера:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]