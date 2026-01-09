[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

По подразумевању, одговори на коментаре највишег нивоа се приказују.

Ово се може подесити тако да корисник мора да кликне "Прикажи одговоре" на коментарима највишег нивоа да би видео подкоментаре.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ово подешавање неће утицати на број коментара највишег нивоа који се иницијално учитавају. Ако имате један коментар највишег нивоа, и 29 подкоментара, са овим подешавањем укљученим ви ћете:

- Видети коментар највишег нивоа.
- Видети "Прикажи одговоре (29)" испод овог коментара.

Ако желите да прикажете све коментаре највишег нивоа у комбинацији са овом опцијом, подесите [почетну страницу на -1](#starting-page).