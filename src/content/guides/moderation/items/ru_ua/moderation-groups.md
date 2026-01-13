Модераторов можно распределять по группам, чтобы модерировать разные страницы или категории контента.

Если модератор принадлежит к одной или нескольким группам, он будет видеть только комментарии из этих групп на странице «Moderate Comments».

Например, представим, что мы управляем сайтом, который отображает видео по категориям. Возможно, мы захотим иметь разных модераторов для видео о кошках, собаках и попугаях, поэтому [добавим эти группы](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Когда мы добавляем модератора, у нас появляется возможность выбрать одну или несколько групп, к которым будет принадлежать модератор:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Наконец, комментарии должны быть привязаны к одной или нескольким группам, чтобы соответствующие модераторы их видели.

Это можно настроить, [добавив несколько групп](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и затем указав соответствующие `Moderation Group` ids в виджете комментариев,
[как указано здесь](/guide-customizations-and-configuration.html#moderation-group-ids).