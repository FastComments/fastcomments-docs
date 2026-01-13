---
Модераторов можно распределять по группам для модерирования различных страниц или категорий контента.

Если модератор принадлежит одной или нескольким группам, он будет видеть только комментарии из этих групп на странице Moderate Comments.

Например, предположим, что мы управляем сайтом, который показывает видео по категориям. Нам может потребоваться назначить разных модераторов для видео о кошках, собаках и попугаях, поэтому [давайте добавим эти группы](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

При добавлении модератора у нас появляется возможность выбрать одну или несколько групп, к которым он будет принадлежать:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Наконец, комментарии должны быть привязаны к одной или нескольким группам, чтобы соответствующие модераторы могли их видеть.

Это можно настроить, [добавив несколько групп](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) и затем указав соответствующие `Moderation Group` ids в виджете комментариев,
[как описано здесь](/guide-customizations-and-configuration.html#moderation-group-ids).

---