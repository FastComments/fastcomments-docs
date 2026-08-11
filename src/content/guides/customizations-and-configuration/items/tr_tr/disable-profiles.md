[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments bir kullanıcının avatarına tıkladığınızda kullanıcının profilini gösterir.

Ancak, bu işlevi devre dışı bırakabiliriz:

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = 'Profilleri Devre Dışı Bırak'; code-example-end]

Bu aynı zamanda kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Profilleri Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='Widget özelleştirme sayfası, Profilleri Devre Dışı Bırak kutucuğu işaretli, böylece avatarlar artık profilleri açmıyor'; title='Profilleri Devre Dışı Bırak' app-screenshot-end]