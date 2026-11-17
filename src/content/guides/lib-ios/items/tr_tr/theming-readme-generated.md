### Tema Ön Ayarları

Dört yerleşik ön ayar mevcuttur:

```swift
// Sistem varsayılanları
sdk.theme = FastCommentsTheme.default

// Gölgeli ve büyük yuvarlatılmış köşeli kartlar
sdk.theme = FastCommentsTheme.modern

// Düz, gölgesiz, küçük köşe yarıçapı, konu çizgisi yok
sdk.theme = FastCommentsThemeTheme.minimal

// Tüm eylem renklerini tek bir marka rengine ayarla
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Yorum Görüntüleme Stilleri

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Ayırıcılı düz liste (varsayılan)
theme.commentStyle = .card    // Gölgeli yuvarlatılmış kartlar
theme.commentStyle = .bubble  // Sohbet balonu stili
```

### Renkler

Tüm renk özellikleri isteğe bağlıdır. Ayarlanmamış değerler makul sistem varsayılanlarına geri döner.

```swift
var theme = FastCommentsTheme()

// Marka renkleri
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Arka planlar
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Eylem düğmeleri
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

### Düzen ve Boşluklandırma

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Yorum satırları arasındaki nokta sayısı
theme.nestingIndent = 20          // Her iç içe geçme seviyesi başına girinti noktası
theme.avatarSize = 36             // Kök yorumlar için avatar çapı
theme.replyAvatarSize = 28        // İç içe yanıtlar için avatar çapı
```

### Görsel Efektler

```swift
theme.showShadows = true          // Kartlarda hafif gölgeler
theme.showThreadLine = true       // İç içe yanıtları birleştiren dikey çizgi
theme.animateVotes = true         // Oy değişikliklerinde yay animasyonu
```

### Temaları Uygulama

İki yaklaşım:

```swift
// SwiftUI ortamı üzerinden (görünüm hiyerarşisi için önerilir)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Doğrudan SDK üzerinde
sdk.theme = theme
```

---

### Anket Renkleri

Anketler, açık ve koyu modda web widget'ının anket kartı renklerini varsayılan olarak kullanır. `primaryColor` ayarlamak ayrıca vurgu rengini ve seçtiğiniz seçeneğin çubuğunu da değiştirir. Bunları doğrudan ayarlamak için:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Özellik | Kullanım amacı |
|----------|----------|
| `pollAccentColor` | Seçili seçenek kenarlığı ve onay işareti, seçilen süre, "Oyunuz kaydedildi" |
| `pollAccentTextColor` | Vurgu rengi üzerinde çizilen onay işareti ve metin |
| `pollOptionBackgroundColor` | Seçenek çipi arka planı |
| `pollOptionPressedBackgroundColor` | Basılıyken seçenek çipi arka planı |
| `pollBarColor` | Seçmediğiniz seçeneklerin sonuç çubuğu |
| `pollSelectedBarColor` | Seçtiğiniz seçeneğin sonuç çubuğu |
| `pollBorderColor` | Onay kutusu işareti ve konturlu hap şekilleri |
| `pollDangerColor` | Hatalar ve "Anketi kaldır" |