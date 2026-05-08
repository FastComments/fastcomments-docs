[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments sayfada kullanıcı listesini göstermez.

Yorum widget'ının yanında, şu anda sayfayı görüntüleyen kişilerin bir listesini görüntüleyebilirsiniz. Liste, kullanıcılar katılıp ayrıldıkça canlı olarak güncellenir ve isimlerini, avatarlarını ve çevrimiçi göstergesini gösterir.

Üç düzen seçeneği vardır:

- `1` - Üst: yorumların üstünde yatay olarak üst üste binen avatarların bir satırı.
- `2` - Sol: widget'ın soluna render edilen isimler ve çevrimiçi noktaları olan bir kenar çubuğu.
- `3` - Sağ: aynı kenar çubuğu widget'ın sağına render edilir.

Bu özelliği etkinleştirmek için **usersListLocation** bayrağını ayarlayın:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Varsayılan olarak liste yalnızca şu anda çevrimiçi olan kullanıcıları gösterir. Geçmişte sayfada yorum yapmış (ancak şu anda sayfayı görüntülemeyen) kişileri de dahil etmek için **usersListIncludeOffline**'ı true olarak ayarlayın:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Geçmişte yorum yapmış kullanıcılar yeşil çevrimiçi noktası olmadan gösterilir; böylece şu anda kimin mevcut olduğu belli olur.

Gizli profillere sahip kullanıcılar, kimlikleri ifşa etmeden sayının doğru kalmasını sağlamak için genel bir avatar ve "Özel Profil" etiketiyle gösterilir.

Bu, kod olmadan da yapılandırılabilir. Widget özelleştirme sayfasında "Kullanıcı Listesi Konumu" seçeneğine bakın. Konum "Kapalı" dışında herhangi bir şeye ayarlandığında, altına "Geçmiş yorumcuları dahil et" onay kutusu görünür.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Canlı kullanıcı sayısı 500'ü aştığında, liste en fazla 30 saniye geride olabilir.