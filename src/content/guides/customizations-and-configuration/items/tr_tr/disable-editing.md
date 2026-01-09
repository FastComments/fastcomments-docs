---
Varsayılan olarak, FastComments kullanıcıların yorumlarını düzenlemesine izin verir.

Ancak, bu engellenebilir.

Widget özelleştirme sayfasında, "Düzenlemeyi Devre Dışı Bırak" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Bu yalnızca normal Commenters kullanıcılarını etkiler; moderators veya admins ise yine düzenleyebilecektir.
- Bu, `contextUserId` iletildiğinde API entegrasyonlarını da etkileyecektir. 

---