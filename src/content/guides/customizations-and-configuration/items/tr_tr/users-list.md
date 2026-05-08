[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments sayfada bir kullanıcı listesi göstermez.

Yorum widget'ının yanında, şu anda sayfayı görüntüleyen kişilerin bir listesini render edebilirsiniz. Liste, kullanıcılar katıldıkça veya ayrıldıkça canlı olarak güncellenir ve onların adını, avatarını ve çevrimiçi göstergesini gösterir.

Üç düzen seçeneği vardır:

- `1` - Üst: yorumların üstünde render edilen üst üste binen avatarların yatay sırası.
- `2` - Sol: widget'ın solunda isimler ve çevrimiçi noktaları gösteren bir kenar çubuğu.
- `3` - Sağ: widget'ın sağında render edilen aynı kenar çubuğu.

Özelliği etkinleştirmek için **usersListLocation** bayrağını ayarlayın:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Varsayılan olarak liste yalnızca şu anda çevrimiçi olan kullanıcıları gösterir. Ayrıca sayfada geçmişte yorum yapmış (ama şu anda sayfayı görüntülemeyen) kişileri de dahil etmek için **usersListIncludeOffline** değerini true olarak ayarlayın:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Geçmiş yorumcular, kimlerin şu anda bulunduğunu netleştirmek için yeşil çevrimiçi noktası olmadan render edilir.

Özel profilli kullanıcılar, kimlikleri açığa çıkarmadan sayımın doğru kalmasını sağlamak için genel bir avatar ve "Private Profile" etiketi ile gösterilir.

Bu, kod olmadan da yapılandırılabilir. Widget özelleştirme sayfasında, "Users List Location" seçeneğine bakın:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

Konum Off dışında herhangi bir değere ayarlandığında, altında "Include past commenters" onay kutusu gösterilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]