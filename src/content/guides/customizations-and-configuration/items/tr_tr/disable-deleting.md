---
Varsayılan olarak, FastComments kullanıcıların yorumlarını silmelerine izin verir.

Ancak, bunu önlemek mümkündür.

Widget özelleştirme sayfasında, "Silme İşlemini Devre Dışı Bırak" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Widget özelleştirme sayfasındaki Silme İşlemini Devre Dışı Bırak seçeneği, yorumcuların yorumlarını kaldırmasını önler'; title='Yorum Silmeyi Devre Dışı Bırak' app-screenshot-end]

- Bu sadece normal Yorumcuları etkiler ve moderatorleri ya da yöneticileri etkilemez; onlar hâlâ silebilir.
- Bu ayrıca `contextUserId` geçirildiğinde API entegrasyonlarını da etkiler. 

---