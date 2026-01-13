### Genel Bakış

FastComments Image Chat, standart FastComments yorum bileşenini genişletir; böylece temel bileşenin tüm yapılandırma seçeneklerini miras alır ve görüntü notlarıyla ilgili birkaç seçenek ekler.

### Gerekli Yapılandırma

#### tenantId

FastComments Tenant ID'niz gereklidir. Bunu [FastComments kontrol panelinizde](https://fastcomments.com/auth/my-account/api-secret) bulabilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat'e Özgü Seçenekler

#### urlId

Varsayılan olarak Image Chat, her konuşma için sayfa URL'si, görüntü kaynağı ve X/Y koordinatlarına dayanarak benzersiz bir tanımlayıcı oluşturur. Bunu özel bir `urlId` ile geçersiz kılabilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Bu, URL yapınız değişebileceği halde aynı konuşmaları korumak istediğinizde veya notları birden fazla sayfa arasında paylaşmak istediğinizde faydalıdır.

#### chatSquarePercentage

Tıklanabilir sohbet işaretçilerinin boyutunu görüntü genişliğinin yüzde olarak kontrol eder. Varsayılan 5%'tir; bu, her işaretçinin görüntü genişliğinin %5'i olduğu anlamına gelir.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Daha büyük, daha görünür işaretçiler
});
```

Daha küçük değerler daha az müdahaleci işaretçiler oluşturur ve ayrıntılı görüntüler için daha iyi çalışır. Daha büyük değerler, yoğun görüntülerde veya mobil cihazlardaki kullanıcılar için işaretçilerin görülmesini ve tıklanmasını kolaylaştırır.

#### hasDarkBackground

Sayfanız koyu arka plana sahipse koyu mod stilini etkinleştirir.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Yorum sayısı değiştiğinde tetiklenen bir geri çağırma fonksiyonu. Rozetler veya sayfa başlıkları gibi UI öğelerini güncellemek için yararlıdır.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Miras Alınan Yapılandırma Seçenekleri

Image Chat standart yorum bileşenini genişlettiği için, temel FastComments bileşeninin herhangi bir yapılandırma seçeneğini kullanabilirsiniz. İşte yaygın olarak kullanılan bazı seçenekler:

#### locale

Bileşenin UI'si için dili ayarlar. FastComments düzinelerce dili destekler.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

Tüm konuşmaları salt okunur yapar. Kullanıcılar mevcut işaretçileri ve tartışmaları görüntüleyebilir, ancak yenisini oluşturamaz veya yanıtlayamaz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Tek Oturum Açma (SSO) kullanarak kimlik doğrulama sisteminizle entegre edin.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO yapılandırması
    }
});
```

Kimlik doğrulama seçenekleri hakkında tam ayrıntılar için SSO belgelerine bakın.

#### maxReplyDepth

Yanıtların kaç seviye derinliğe kadar olabileceğini kontrol edin. Varsayılan olarak Image Chat bunu 0 olarak ayarlar; bu, tüm yorumların düz olduğu (iç içe yanıt yok) anlamına gelir. Konuşmaların dallanmasını istiyorsanız bunu değiştirebilirsiniz.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 seviye iç içe izni ver
});
```

### Dahili Yapılandırma

Bu seçenekler Image Chat tarafından otomatik olarak ayarlanır ve üzerine yazılmamalıdır:

`productId` Image Chat için otomatik olarak `2` olarak ayarlanır. Sohbet penceresi işlevselliği sağlamak için `floating-chat` uzantısı otomatik olarak yüklenir. Bileşen, mobil cihazları (genişliği 768px'den az ekranlar) otomatik olarak algılar ve UI'yi buna göre tam ekran sohbet pencereleriyle ayarlar.

### Hedef Öğe Esnekliği

`FastCommentsImageChat`'e verilen ilk parametre ya doğrudan bir `<img>` elementi ya da içinde bir görüntü bulunan bir konteyner elementi olabilir:

```javascript
// Doğrudan görüntü elementi
FastCommentsImageChat(document.getElementById('my-image'), config);

// İçinde görüntü olan konteyner
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Bir konteyner elementi geçirirseniz bileşen görüntüyü otomatik olarak bulacaktır.

### Tam Örnek

İşte birden fazla yapılandırma seçeneğini bir arada gösteren bir örnek:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // SSO yapılandırmanız
    },
    maxReplyDepth: 1
});
```

Temel bileşenden miras alınan tüm kullanılabilir yapılandırma seçeneklerinin eksiksiz listesi için ana FastComments yapılandırma belgelerine bakın.