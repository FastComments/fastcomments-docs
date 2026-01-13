Site Analytics veya kontrol panelinde sadece `Analytics` olarak adlandırılan bu özellik, topluluğunuzun tüm alan adlarınızda FastComments'i nasıl kullandığına dair bir genel bakış sağlar.

FastComments, birçok platformun sunmadığı benzersiz özellikler sunar; her sayfadaki çevrimiçi kullanıcıların **canlı** raporlanması ve sayfaların çevrimiçi kullanıcı sayısına göre sıralanması gibi. Bunu yapmak için [Analytics Page](https://fastcomments.com/auth/my-account/analytics)'i ziyaret edin ve `Top Pages` altında `Sort by users online`'a tıklayın.

Hem toplam `Users Online` hem de `Top Pages` metrikleri canlıdır ve gecikmesiz raporlanır.

`Top Pages` varsayılan olarak her sayfadaki yorum sayısına göre sıralayacaktır.

Son olarak, kiracınız genelinde toplam metriklerin günlük ve zaman içindeki dökümü şunlar için sağlanır:

- Page Loads
  - Bu, bir kullanıcının bir veya daha fazla FastComments widget'ı içeren bir sayfayı açtığı sayıdır. Sayfa birden fazla widget içeriyorsa, bu sayı o sayfadaki widget sayısı kadar artırılacaktır. SPA'nız varsa, uygulama her yeni yorum dizisi açtığında bu sayı artacaktır. Bu, React Native kütüphanesi için de geçerlidir.
  - Bu metrik ayrıca Flex planlarında faturalandırma amaçları için kullanılır.
- Comments Left
  - Bu, doğrulama veya onay durumundan bağımsız olarak veya spam olup olmadıklarına bakılmaksızın tüm yorumları içerir.
- Votes Left
  - Bu, bırakılan oy sayısı içindir. Anonim oylama etkinleştirilmediği sürece yalnızca doğrulanmış oyları sayacaktır.
- Accounts Created
  - Bu metrik, yeni bir SSO kullanıcısı eklendiğinde veya bir yorumcu sitenizi kullanarak FastComments ile ilk kez yorum yaptığında için geçerlidir.

Bu metrikler neredeyse gerçek zamanlıdır ve bir dakikaya kadar gecikebilir.
