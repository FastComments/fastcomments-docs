[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

По подразумевању, одговори на коментаре првог нивоа се приказују.

Ово се може конфигурисати тако да корисник мора кликнути "Прикажи одговоре" на коментарима првог нивоа да би видио подкоментаре.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ово подешавање неће утицати на број коментара првог нивоа који се првобитно учитавају. Ако имате један коментар првог нивоа и 29 подкоментара, са овим подешавањем укљученим, ви ћете:

- Видите коментар првог нивоа.
- Видите "Прикажи одговоре (29)" испод овог коментара.

Ако желите приказати све коментаре првог нивоа у комбинацији с овом опцијом, поставите [почетну страницу на -1](#starting-page).

---