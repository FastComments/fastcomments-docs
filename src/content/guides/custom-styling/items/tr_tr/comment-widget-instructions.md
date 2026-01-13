## Yorum bileşeni stillerini nasıl özelleştirirsiniz

Yorum bileşeni stilini iki şekilde özelleştirebilirsiniz:

### Seçenek 1: customCSS Parametresi ile

Özel CSS'inizi widget'ı başlatırken `customCSS` parametresine bir dize olarak iletin:

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### Seçenek 2: Yönetici Paneli Üzerinden

1. Yönetici panelinizdeki [Widget Özelleştirme sayfasına](https://fastcomments.com/auth/my-account/customize-widget) gidin  
2. "Gelişmiş" altında "Özel CSS" bölümüne gidin  
3. Özel CSS'inizi girin  
4. "Kaydet" seçeneğine tıklayın

Özel CSS'iniz sitenizdeki tüm yorum widget'larına uygulanacaktır.

## İpuçları

- Gerekirse varsayılan stilleri geçersiz kılmak için `!important` kullanın  
- Diğer site bölümlerini etkilememek için belirli seçicileri hedefleyin  
- Uyumluluk için CSS'inizi farklı tarayıcılarda test edin  
- Widget standart CSS kullanır - özel önişlemcilere gerek yoktur