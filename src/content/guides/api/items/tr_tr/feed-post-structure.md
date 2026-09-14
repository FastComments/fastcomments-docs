A `FeedPost` nesnesi, FastComments akışındaki bir gönderiyi temsil eder. Bir akış, kendi yorum dizileriyle birlikte gönderilerin bir akışı olup, Feed widget'ı tarafından render edilir. Her gönderinin bir yazarı, isteğe bağlı zengin içeriği, medyası ve bağlantıları vardır ve bir akışın filtrelenebilmesi için etiketlenebilir.

`FeedPost` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'FeedPost Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** Gönderiyi oluşturan FastComments veya SSO kullanıcısının kimliği. **/
    fromUserId?: string
    /** Ayarlanmamışsa kullanıcıdan doldurulur. **/
    fromUserDisplayName?: string | null
    /** READONLY. Kullanıcıdan doldurulur. **/
    fromUserAvatar?: string | null
    /** Bir akışı filtrelemek için kullanılır. **/
    tags?: string[]
    /** Bir akış içinde sıralama ağırlığı. Daha yüksek değerler önce sıralanır. **/
    weight?: number
    /** Kendi kullanımınız için serbest biçimli anahtar/değer çiftleri. **/
    meta?: Record<string, string>
    /** Temizlenmiş HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Sayılacak reaksiyon türü. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** Medya öğesine tıklandığında yönlendirilen adres. **/
    linkUrl?: string
    /** Her bir versiyon için bir giriş. Widget en uygun olanı seçer. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** Bağlantı metni, örneğin "Şimdi kaydol". **/
    text?: string
    /** Bağlantıyla birlikte gösterilen bir başlık. **/
    title?: string
    /** Bağlantıyla birlikte gösterilen bir açıklama. **/
    description?: string
    url?: string
}
[inline-code-end]

Notlar:

- Bu alanların bazıları `READONLY` olarak işaretlenmiştir - bunlar API tarafından döndürülür ancak ayarlanamaz.
- Bir gönderideki yorumlar, `urlId`'si `post:` ve ardından gönderinin `_id`'si gelen normal yorumlardır. Bu değeri Yorum API'si ile kullanarak bir gönderide yorum okuyabilir veya oluşturabilirsiniz.