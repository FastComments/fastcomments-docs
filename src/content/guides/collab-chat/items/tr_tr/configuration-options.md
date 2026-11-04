### Genel Bakış

FastComments Collab Chat, standart FastComments yorum widget'ını genişletir, böylece temel widget'ın tüm yapılandırma seçeneklerini devralır ve metin açıklamalarına özgü birkaç seçenek ekler.

### Gerekli Yapılandırma

#### tenantId

FastComments Tenant ID'niz gereklidir. Bunu [FastComments panonuzda](https://fastcomments.com/auth/my-account/api-secret) bulabilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat'e Özgü Seçenekler

#### urlId

Varsayılan olarak, Collab Chat her konuşma için sayfa URL'sine, öğeye giden DOM yoluna ve seçili metin aralığına dayalı benzersiz bir tanımlayıcı oluşturur. Bunu özel bir `urlId` ile geçersiz kılabilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Bu, URL yapınız değişebileceğinde ancak aynı konuşmaları korumak istediğinizde veya açıklamaları birden fazla sayfada paylaşmak istediğinizde kullanışlıdır.

#### topBarTarget

Kullanıcı sayısını ve tartışma sayısını gösteren üst çubuğun görüntülenmesini kontrol eder. Üst çubuğu tamamen devre dışı bırakmak için `null` olarak ayarlayın veya belirli bir konumda görüntülemek için bir DOM öğesi sağlayın.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Sayfanız koyu bir arka plana sahip olduğunda koyu mod stilini etkinleştirin. Bu algılama otomatiktir, ancak geçersiz kılmak istenebilir.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Yorum sayısı her değiştiğinde tetiklenen bir geri çağırma işlevi. Bu, rozetler veya sayfa başlıkları gibi UI öğelerini güncellemek için kullanışlıdır.

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

### Devralınan Yapılandırma Seçenekleri

Collab Chat standart yorum widget'ını genişlettiğinden, temel FastComments widget'ından herhangi bir yapılandırma seçeneğini kullanabilirsiniz. İşte yaygın olarak kullanılan bazı seçenekler:

#### locale

Widget UI'si için dili ayarlayın. FastComments düzinelerce dili destekler.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Tüm konuşmaları salt okunur yapın. Kullanıcılar mevcut açıklamaları görüntüleyebilir ancak yenilerini oluşturamaz veya yanıt veremez.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso ve simpleSSO

Tek Oturum Açma (Single Sign-On) kullanarak kimlik doğrulama sisteminizle entegre olun.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Kimlik doğrulama seçenekleriyle ilgili tüm ayrıntılar için SSO belgelerine bakın.

#### maxReplyDepth

Yanıtların kaç seviye derine gidebileceğini kontrol edin. Varsayılan olarak, Collab Chat bunu 0 olarak ayarlar, yani tüm yorumlar düzdür (iç içe yanıtlar yok). Dallanmış konuşmalar istiyorsanız bunu değiştirebilirsiniz.

[inline-code-attrs-start title = "Yapılandırma Örneği"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Dahili Yapılandırma

Bu seçenekler Collab Chat tarafından otomatik olarak ayarlanır ve geçersiz kılınmamalıdır:

`productId`, Collab Chat için otomatik olarak `3` olarak ayarlanır. Sohbet penceresi işlevselliğini sağlamak için `floating-chat` uzantısı otomatik olarak yüklenir. Widget, mobil cihazları (768px'den dar ekranlar) otomatik olarak algılar ve UI'yi buna göre ayarlar.

### Tam Örnek

Birden fazla yapılandırma seçeneğini birlikte gösteren bir örnek:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Temel widget'tan devralınan tüm kullanılabilir yapılandırma seçeneklerinin tam listesi için ana FastComments yapılandırma belgelerine bakın.