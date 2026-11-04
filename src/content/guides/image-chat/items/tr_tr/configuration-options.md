### Genel Bakış

FastComments Image Chat, standart FastComments yorum widget'ını genişletir; böylece temel widget'ın tüm yapılandırma seçeneklerini devralır ve görüntü ek açıklamalarına özgü birkaç seçenek ekler.

### Gerekli Yapılandırma

#### tenantId

FastComments Tenant ID'niz gereklidir. Bunu [FastComments panonuzda](https://fastcomments.com/auth/my-account/api-secret) bulabilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat'e Özgü Seçenekler

#### urlId

Varsayılan olarak, Image Chat her konuşma için sayfa URL'si, görüntü kaynağı ve X/Y koordinatlarına dayalı benzersiz bir tanımlayıcı oluşturur. Bunu özel bir `urlId` ile geçersiz kılabilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Bu, URL yapınız değişebileceğinde ancak aynı konuşmaları korumak istediğinizde veya ek açıklamaları birden fazla sayfada paylaşmak istediğinizde kullanışlıdır.

#### chatSquarePercentage

Tıklanabilir sohbet işaretçilerinin boyutunu, görüntü genişliğinin yüzdesi olarak kontrol eder. Varsayılan %5'tir, yani her işaretçi görüntü genişliğinin %5'idir.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Daha büyük, daha görünür işaretçiler
});
```

Daha küçük değerler, detaylı görüntüler için daha iyi çalışan, daha az dikkat dağıtan işaretçiler oluşturur. Daha büyük değerler, işaretçileri yoğun görüntülerde veya mobil cihaz kullanan kullanıcılar için daha kolay görünür ve tıklanabilir hale getirir.

#### hasDarkBackground

Sayfanız koyu bir arka plana sahipken koyu mod stili etkinleştirin.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Yorum sayısı her değiştiğinde tetiklenen bir geri çağırma fonksiyonu. Bu, rozetler veya sayfa başlıkları gibi UI öğelerini güncellemek için kullanışlıdır.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Toplam yorum:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Devralınan Yapılandırma Seçenekleri

Image Chat standart yorum widget'ını genişlettiğinden, temel FastComments widget'ından herhangi bir yapılandırma seçeneğini kullanabilirsiniz. İşte yaygın olarak kullanılan bazı seçenekler:

#### locale

Widget UI'sinin dilini ayarlayın. FastComments onlarca dili destekler.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // İspanyolca
});
```

#### readonly

Tüm konuşmaları salt okunur yapın. Kullanıcılar mevcut işaretçileri ve tartışmaları görüntüleyebilir ancak yenilerini oluşturamaz veya yanıt veremez.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso ve simpleSSO

Tek Oturum Açma (Single Sign-On) kullanarak kimlik doğrulama sisteminizle entegre olun.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO yapılandırması
    }
});
```

Kimlik doğrulama seçenekleriyle ilgili tüm ayrıntılar için SSO belgelerine bakın.

#### maxReplyDepth

Yanıtların kaç seviye derine gidebileceğini kontrol edin. Varsayılan olarak, Image Chat bunu 0 olarak ayarlar, yani tüm yorumlar düzdür (iç içe yanıtlar yok). Dallı konuşmalar istiyorsanız bunu değiştirebilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 seviyeye kadar iç içe geçmeye izin ver
});
```

### Dahili Yapılandırma

Bu seçenekler Image Chat tarafından otomatik olarak ayarlanır ve geçersiz kılınmamalıdır:

`productId`, Image Chat için otomatik olarak `2` olarak ayarlanır. Sohbet penceresi işlevselliğini sağlamak için `floating-chat` uzantısı otomatik olarak yüklenir. Widget, mobil cihazları (768px'den dar ekranlar) otomatik olarak algılar ve UI'yi buna göre tam ekran sohbet pencereleriyle uyarlar.

### Hedef Öğe Esnekliği

`FastCommentsImageChat`'in ilk parametresi, doğrudan bir `<img>` öğesi veya içinde görüntü bulunan bir kapsayıcı öğe olabilir:

```javascript
// Doğrudan görüntü öğesi
FastCommentsImageChat(document.getElementById('my-image'), config);

// İçinde görüntü bulunan kapsayıcı
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Bir kapsayıcı öğe geçirirseniz widget, görüntüyü otomatik olarak bulur.

### Tam Örnek

Birden fazla yapılandırma seçeneğini birlikte gösteren bir örnek:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Ürün Fotoğrafı` : 'Ürün Fotoğrafı';
    },
    sso: {
        // SSO yapılandırmanız
    },
    maxReplyDepth: 1
});
```

Temel widget'tan devralınan tüm kullanılabilir yapılandırma seçeneklerinin tam listesi için ana FastComments yapılandırma belgelerine bakın.