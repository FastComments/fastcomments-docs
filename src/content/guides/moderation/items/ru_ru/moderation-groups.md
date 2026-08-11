---
Модераторы могут быть размещены в группы для модерации разных страниц или категорий контента.

Когда модератор принадлежит одной или нескольким группам, он будет видеть только комментарии из этих групп на странице модерации комментариев.

Например, предположим, что мы управляем сайтом, который отображает видео по категориям. Мы можем захотеть иметь разных модераторов для видео о кошках, собаках и попугаях, поэтому [добавим эти группы](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Список групп модерации с группами Cat, Dog и Parrot, созданными для каждой категории видео'; title='Страница групп модерации' app-screenshot-end]

Когда мы добавляем модератора, у нас появляется возможность выбрать одну или несколько групп, к которым будет принадлежать модератор:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Форма добавления модератора с селектором группы, используемым для назначения модератора в одну или несколько групп'; title='Добавление модератора и выбор группы' app-screenshot-end]

Наконец, комментарии должны быть привязаны к одной или нескольким группам, чтобы нужные модераторы их видели.

Это можно настроить, [добавив некоторые группы](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) а затем указав соответствующие идентификаторы `Moderation Group` в виджете комментариев,
[как указано здесь](/guide-customizations-and-configuration.html#moderation-group-ids).

---