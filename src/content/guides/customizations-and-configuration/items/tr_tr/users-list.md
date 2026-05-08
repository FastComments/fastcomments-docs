[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Varsayılan olarak FastComments sayfada kullanıcı listesini göstermez.

Yorum bileşeninin yanında, şu anda sayfayı görüntüleyen kişilerin bir listesini görüntüleyebilirsiniz. Liste, kullanıcılar katıldıkça ve ayrıldıkça canlı olarak güncellenir; isimlerini, avatarlarını ve çevrimiçi göstergesini gösterir.

Üç yerleşim seçeneği vardır:

- `1` - Top: yorumların üzerinde görüntülenen üst üste binen avatarlardan oluşan yatay sıra.
- `2` - Left: yorum bileşeninin solunda görüntülenen, isimler ve çevrimiçi noktaları olan bir kenar çubuğu.
- `3` - Right: aynı kenar çubuğu yorum bileşeninin sağında görüntülenir.

Özelliği etkinleştirmek için **usersListLocation** bayrağını ayarlayın:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Varsayılan olarak liste yalnızca şu anda çevrimiçi olan kullanıcıları gösterir. Sayfaya geçmişte yorum yapmış (ama şu anda görüntülemeyen) kişileri de dahil etmek için **usersListIncludeOffline**'ı true olarak ayarlayın:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Geçmiş yorumcular yeşil çevrimiçi noktası olmadan görüntülenir, böylece kimin şu anda mevcut olduğu açıkça görülür.

Özel profilli kullanıcılar, kimliklerini açığa çıkarmadan sayımın doğru kalmasını sağlamak için genel bir avatar ve "Özel Profil" etiketiyle gösterilir.

Bu, kod yazmadan da yapılandırılabilir. Widget özelleştirme sayfasında "Kullanıcı Listesi Konumu" seçeneğine bakın. Konum Off dışında bir değere ayarlandığında, altında "Geçmiş yorumcuları dahil et" onay kutusu görünür.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]