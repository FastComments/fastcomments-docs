## Yorum Widget Stillerini Nasıl Özelleştirebilirsiniz

Yorum widget stilini iki şekilde özelleştirebilirsiniz:

### Seçenek 1: `customCSS` Parametresi ile

Widget'ı başlatırken özel CSS'inizi `customCSS` parametresine bir dize olarak iletin:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Seçenek 2: Yönetici Paneli Üzerinden

1. Yönetici panelinizdeki [Widget Özelleştirme sayfasına](https://fastcomments.com/auth/my-account/customize-widget) gidin
2. "Gelişmiş" altındaki "Özel CSS" bölümüne gidin
3. Özel CSS'inizi girin
4. "Kaydet"e tıklayın

Özel CSS'iniz sitenizdeki tüm yorum widget'larına uygulanacaktır.

## İpuçları

- `!important` gerektiğinde varsayılan stillerin üzerine yazmak için kullanın
- Diğer site bölümlerini etkilememek için belirli seçicileri hedefleyin
- Uyumluluk için CSS'inizi farklı tarayıcılarda test edin
- Widget standart CSS kullanır - özel bir ön işlemci gerekmez