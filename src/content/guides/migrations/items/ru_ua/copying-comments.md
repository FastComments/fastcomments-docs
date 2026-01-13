В случае, если данные нужно переместить, FastComments предоставляет инструмент самообслуживания для перемещения комментариев
между страницами и статьями.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Заполнение полей "From"

Чтобы определить, откуда перемещать комментарии, нам просто нужно знать исходный `URL ID`.

Если вы не передаёте значение `urlId` в конфигурации виджета комментариев, то это будет "clean" версия URL страницы.

Вы можете увидеть, какие значения имеют ваши комментарии для `URL ID`, экспортировав их.

### Заполнение полей "To"

Чтобы определить, куда перемещать комментарии, нам нужно знать целевой `URL ID` и `URL`.

The `URL ID` will be the bucket that the comment goes in. The `URL` field is used so that you can navigate directly
to the comment from emails and moderation tools.

#### WordPress

Если вы используете WordPress, вы, например, введёте идентификаторы статей в поля `URL ID` To/From в инструменте миграции,
а не URL.

---