---
Varsayılan olarak, FastComments kullanıcıların yorumlarını düzenlemelerine izin verir.

Ancak, bunu önlemek mümkündür.

Widget özelleştirme sayfasında, "Disable Editing" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Widget özelleştirme sayfasındaki Düzenlemeyi Devre Dışı Bırak seçeneği, yorumcuların yorumlarını düzenlemelerini engeller'; title='Yorum Düzenlemeyi Devre Dışı Bırak' app-screenshot-end]

- Bu sadece normal Yorumcuları etkiler ve moderatörleri ya da yöneticileri etkilemez; onlar hâlâ düzenleyebilir.
- Bu ayrıca `contextUserId` geçirildiğinde API entegrasyonlarını da etkiler. 

---