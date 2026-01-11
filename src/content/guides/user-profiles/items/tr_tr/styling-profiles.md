Kullanıcı Profilleri sitenizin bağlamında (yorum widget'ı aracılığıyla) açıldığında, FastComments widget'ınıza uyguladığınız herhangi bir özel CSS stili otomatik olarak profil modaline enjekte edilir.

### Nasıl Çalışır

Bir kullanıcı yorum widget'ınızdaki bir profil bağlantısına tıkladığında, `.fast-comments-profile` sınıfına sahip bir profil modalı açılır. Widget'ınıza ait özel CSS, profil görünümüne otomatik olarak enjekte edilir. Yorum widget'ınızı zaten stilize ettiyseniz, bu stiller profillere de uygulanacaktır.

### CSS Sınıfları

FastComments profilleri sınıf tabanlı bir CSS mimarisi kullanır. CSS özel özellikleri (custom properties) kullanılmaz.

Ana profil sayfası kök konteyner olarak `.user-profile` kullanır. Başlık bölümü `.profile-header` olup arka plan resmi için `.profile-header-background` kullanılır. Profil içeriği `.profile-content` içinde yer alır.

Avatar `.profile-avatar` ve `.profile-avatar-wrapper` kullanır. Kullanıcının adı `.profile-name` ve biyografi metni `.profile-bio`'dur. İstatistikler `.profile-stats` içinde bulunur ve tekil istatistikler `.stat` kullanır.

Sosyal bağlantılar `.profile-social-links` içinde yer alır ve tekil bağlantılar `.social-link` olarak tanımlanır. Rozetler `.profile-badges` ve `.badge` kullanır. Rozet ilerleme çubukları `.progress-outer` ve `.progress-bar` kullanır.

Sekmeler konteyner için `.profile-tabs`, tekil sekmeler için `.tab` ve seçili sekme için `.tab.active` kullanır. Sekme içeriği `.tab-body` ve `.tab-body.active` kullanır. Sekmelerdeki bildirim sayıları `.tab .count` ile gösterilir.

Bildirimler `.notification` kullanır ve DM konuşmaları `.conversation` kullanır. Çevrimiçi durum `.activity-indicator` ile gösterilir ve aktif durum için `.activity-indicator.online` kullanılır. Okunmamış sayacı `.unread-count` kullanır.

Profil modal konteyneri `.fast-comments-profile` olup kapatma düğmesi için `.fast-comments-profile-close` kullanılır.

### Karanlık Mod

Karanlık mod `.user-profile` üzerinde `.dark` sınıf değiştiricisini kullanır.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Örnekler

**Başlık:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Rozetler:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Sekmeler:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```