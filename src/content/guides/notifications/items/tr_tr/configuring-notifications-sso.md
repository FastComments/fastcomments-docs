SSO için bildirimler söz konusu olduğunda göz önünde bulundurulması gereken yapılandırmalar şunlardır:

- Kullanıcının bildirimlere abone olup olmadığı.
  - Bu, `SSOUser` nesnesinde `optedInNotifications` bayrağını `true` veya `false` olarak ayarlayarak yapılır.
  - Bu API aracılığıyla ayarlanabilir.
  - Ayrıca, bu bayrak için payload içinde bir değer gönderirseniz, kullanıcı bir yorum dizisini yüklediğinde otomatik olarak güncellenecektir.
- Kullanıcının **abonelik** bildirimlerine abone olup olmadığı.
  - Bu, `SSOUser` nesnesinde `optedInSubscriptionNotifications` bayrağını `true` veya `false` olarak ayarlayarak yapılır.
  - Bu API aracılığıyla ayarlanabilir.
  - Ayrıca, bu bayrak için payload içinde bir değer gönderirseniz, kullanıcı bir yorum dizisini yüklediğinde otomatik olarak güncellenecektir.
- E-posta adresinin tanımlanması.
  - Mevcut değilse, e-posta tabanlı bildirimler gönderemeyiz.
- E-postalardaki abonelikten çıkış bağlantılarını devre dışı bırakıp bırakmama.
  - Bu, `Tenant` nesnesindeki `disableUnsubscribeLinks` bayrağı ile yapılır.
  - Bu API aracılığıyla ayarlanabilir.
- Özel bir abonelikten çıkış bağlantısı kullanıp kullanmama.
  - Bu, `DomainConfig` nesnesindeki `footerUnsubscribeURL` özelliği ile yapılır.
  - Bu API aracılığıyla ayarlanabilir.
  - Ayrıca aynı nesnedeki `emailHeaders` aracılığıyla ilgili abonelikten çıkış başlıklarını ayarlamayı da düşünebilirsiniz.

---