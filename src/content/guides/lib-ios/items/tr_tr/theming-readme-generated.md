### Tema Ön Ayarlar

Dört yerleşik ön ayar mevcuttur:

```swift
// Sistem varsayılanları
sdk.theme = FastCommentsTheme.default

// Gölgelere ve büyük yuvarlatılmış köşelere sahip kartlar
sdk.theme = FastCommentsTheme.modern

// Düz, gölge yok, küçük köşe yarıçapı, konu çizgisi yok
sdk.theme = FastCommentsTheme.minimal

// Tüm işlem renklerini tek bir marka rengi yap
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Yorum Görüntüleme Stilleri

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Bölücülerle düz liste (varsayılan)
theme.commentStyle = .card    // Gölgelere sahip yuvarlatılmış kartlar
theme.commentStyle = .bubble  // Sohbet balonu stili
```

### Renkler

Tüm renk özellikleri isteğe bağlıdır. Ayarlanmamış değerler uygun sistem varsayılanlarına geri döner.

```swift
var theme = FastCommentsTheme()

// Marka renkleri
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Arka planlar
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// İşlem düğmeleri
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Oylar
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Bağlantılar
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Diyaloglar
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Giriş çubuğu
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Diğer
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Tipografi

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Düzen ve Aralık

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Yorum satırları arasındaki mesafe (puan)
theme.nestingIndent = 20          // Her iç içe seviye için girinti (puan cinsinden)
theme.avatarSize = 36             // Kök yorumlar için avatar çapı
theme.replyAvatarSize = 28        // İç içe yanıtlar için avatar çapı
```

### Görsel Efektler

```swift
theme.showShadows = true          // Kartlarda hafif gölgeler
theme.showThreadLine = true       // İç içe yanıtları bağlayan dikey çizgi
theme.animateVotes = true         // Oy değişikliklerinde yay animasyonu
```

### Tema Uygulama Yöntemleri

İki yaklaşım:

```swift
// SwiftUI ortamı aracılığıyla (görünüm hiyerarşisi için önerilir)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Doğrudan SDK üzerinde
sdk.theme = theme
```

---
---