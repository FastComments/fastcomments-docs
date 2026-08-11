[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments sayfada bir kullanıcı listesi göstermez.

Yorum widget'ının yanında, sayfayı şu anda görüntüleyen kişilerin bir listesini render edebilirsiniz. Liste, kullanıcılar katıldıkça ve ayrıldıkça canlı olarak güncellenir ve adlarını, avatarlarını ve çevrimiçi göstergesini gösterir.

Üç düzen seçeneği vardır:

- `1` - Üst: Yorumların üzerinde render edilen, üst üste binen avatarların yatay bir satırı.
- `2` - Sol: Widget'ın solunda render edilen, isimler ve çevrimiçi noktalar içeren bir kenar çubuğu.
- `3` - Sağ: Widget'ın sağında render edilen aynı kenar çubuğu.

**usersListLocation** bayrağını ayarlayarak özelliği etkinleştirin:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Varsayılan olarak liste yalnızca şu anda çevrimiçi olan kullanıcıları gösterir. Geçmişte sayfada yorum yapmış (ancak şu anda görüntülemeyen) kişileri de dahil etmek için **usersListIncludeOffline** değerini true olarak ayarlayın:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Geçmiş yorumcular, yeşil çevrimiçi nokta olmadan render edilir, böylece şu anda kimlerin bulunduğu net olur.

Özel profillere sahip kullanıcılar, kimlikleri ortaya çıkmadan sayımın doğru kalmasını sağlamak için genel bir avatar ve "Özel Profil" etiketiyle gösterilir.

Bu aynı zamanda kod olmadan da yapılandırılabilir. Widget özelleştirme sayfasında "Users List Location" seçeneğine bakın. Konum Off dışındaki bir değere ayarlandığında, altında bir "Geçmiş yorumcuları dahil et" onay kutusu görünür.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Kullanıcı Listesi Konumu Sağ olarak ayarlandı, altında geçmiş yorumcuları dahil et onay kutusu gösteriliyor'; title='Kullanıcı Listesi Ayarları'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

500 canlı kullanıcıdan sonra, liste en fazla 30 saniye gecikmeli olabilir.