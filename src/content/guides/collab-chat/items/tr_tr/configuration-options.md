### Overview

FastComments Collab Chat, standart FastComments yorum bileşenini genişletir; böylece temel bileşenden tüm yapılandırma seçeneklerini miras alır ve metin açıklamalarına özgü birkaç seçenek ekler.

### Required Configuration

#### tenantId

FastComments Tenant ID'niz gereklidir. Bunu [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret) sayfanızda bulabilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specific Options

#### urlId

Varsayılan olarak, Collab Chat her konuşma için sayfa URL'si, öğeye giden DOM yolu ve seçili metin aralığına dayanarak benzersiz bir tanımlayıcı oluşturur. Bunu özel bir `urlId` ile geçersiz kılabilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Bu, URL yapınız değişse bile aynı konuşmaları korumak istediğinizde veya açıklamaları birden çok sayfa arasında paylaşmak istediğinizde kullanışlıdır.

#### topBarTarget

Kullanıcı sayısını ve tartışma sayısını gösteren üst çubuğun görüntülenmesini kontrol eder. Üst çubuğu tamamen devre dışı bırakmak için `null` olarak ayarlayın veya belirli bir konuma render etmek için bir DOM öğesi sağlayın.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Üst çubuğu devre dışı bırak
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Üst çubuğu özel bir konumda göster
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Sayfanız koyu bir arka plana sahip olduğunda koyu mod stilini etkinleştirin. Bu tespit otomatik olarak yapılır, ancak üzerine yazmak isteyebilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Yorum sayısı her değiştiğinde tetiklenen bir geri çağırma (callback) fonksiyonu. Rozetler veya sayfa başlıkları gibi UI öğelerini güncellemek için faydalıdır.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Inherited Configuration Options

Collab Chat, standart yorum bileşenini genişlettiği için temel FastComments bileşeninden herhangi bir yapılandırma seçeneğini kullanabilirsiniz. İşte yaygın olarak kullanılan bazı seçenekler:

#### locale

Bileşen UI'sı için dili ayarlayın. FastComments onlarca dili destekler.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // İspanyolca
});
[inline-code-end]

#### readonly

Tüm konuşmaları salt okunur yapın. Kullanıcılar mevcut açıklamaları görüntüleyebilir ancak yeni oluşturamaz veya yanıtlayamaz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Tek Oturum Açma (Single Sign-On) kullanarak kimlik doğrulama sisteminizle entegrasyon sağlayın.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO yapılandırması
    }
});
[inline-code-end]

Kimlik doğrulama seçenekleri hakkında tam detay için SSO belgelerine bakın.

#### maxReplyDepth

Yanıtların kaç seviyeye kadar iç içe olabileceğini kontrol edin. Varsayılan olarak, Collab Chat bunu 0 olarak ayarlar; bu, tüm yorumların düz olduğu (iç içe yanıt yok) anlamına gelir. Konu dizileri (threaded conversations) istiyorsanız bunu değiştirebilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 seviyeye kadar iç içe izin ver
});
[inline-code-end]

### Internal Configuration

Bu seçenekler Collab Chat tarafından otomatik olarak ayarlanır ve üzerine yazılmamalıdır:

`productId` Collab Chat için otomatik olarak `3` olarak ayarlanır. `floating-chat` uzantısı sohbet penceresi işlevselliğini sağlamak için otomatik olarak yüklenir. Bileşen, mobil cihazları (genişliği 768px'in altında olan ekranlar) otomatik olarak algılar ve UI'ı buna göre ayarlar.

### Complete Example

Aşağıda birden çok yapılandırma seçeneğini bir arada gösteren bir örnek bulunmaktadır:

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // SSO yapılandırmanız
    },
    maxReplyDepth: 1
});
[inline-code-end]

Tüm kullanılabilir yapılandırma seçeneklerinin tam listesi için, temel FastComments yapılandırma belgelerine bakın.