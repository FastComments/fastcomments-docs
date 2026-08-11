[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı profillerinde bir "Profil Yorumları" sekmesi gösterir ve ziyaretçilerin birinin profiline yorum bırakmasına izin verir.

Ancak, bu sekmeyi devre dışı bırakabiliriz:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Profil Yorumlarını Devre Dışı Bırak'; code-example-end]

Bu aynı zamanda kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Profil Yorumlarını Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='Profil yorumları sekmesini gizlemek için Profil Yorumlarını Devre Dışı Bırak onay kutusu işaretlenmiş widget özelleştirme sayfası'; title='Profil Yorumlarını Devre Dışı Bırak' app-screenshot-end]