A `Poll` bir yorumla ilişkilendirilir, kendi başına ayrı bir nesne değildir. Yorumla birlikte oluşturulur (`POST /api/v1/comments` bakınız), ya da daha sonra mevcut bir yoruma `PUT /api/v1/polls/:commentId` ile eklenir.

Oy sayıları anketin kendisinde tutulur, bu yüzden bir anketi okuduğunuzda sonuçları toplamanıza gerek kalmaz. Bu sayıları oluşturan bireysel oylar `PollVote` nesneleridir.

Her seçenek, anket oluşturulduğunda oluşturulan bir `id`'ye sahiptir. Bu id, oy vermek, bir seçeneğin etiketini değiştirmek ve anketi seçenekler eklenip çıkarıldığında `PUT` ettiğinizde bir seçeneği (ve oylarını) tutmak için kullanılır. Bir seçeneğe referans vermenin tek güvenli yolu budur - listenin konumuna asla başvurmayın.

[inline-code-attrs-start title = 'Anket Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** Ayarlandığında ve geçmişte ise, anket kapanır ve artık oy kabul etmez. **/
    closesAt?: string | null
    /** 0 anonim (varsayılan), 1 yöneticiler ve moderatörler, 2 herkes. Yoksa anonim anlamına gelir. **/
    privacy?: 0 | 1 | 2 | null
    /** true olduğunda, sayılar henüz oy vermemiş olan herkes tarafından gizlenir. Yoksa false anlamına gelir. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Sınırlamalar

- Bir soru zorunludur ve en fazla 200 karakter olabilir.
- Bir anketin 2 ile 10 arasında seçeneği olabilir.
- Bir seçenek etiketi zorunludur, en fazla 100 karakter olabilir ve anket içinde (büyük/küçük harf duyarsız) benzersiz olmalıdır.
- `closesAt` anket oluşturulduğunda gelecekte olmalıdır. Bir anketi hemen kapatmak için, geçmiş bir tarih ile `PATCH` yapın.

### Site Ayarları

Anketler, Özelleştir Widget altında değiştirebileceğiniz site yapılandırmanıza uyar:

- Bir anket oluşturulmadan önce anketlerin etkinleştirilmiş olması gerekir, aksi takdirde API `polls-disabled` yanıtını verir.
- Oy verme, oturum açmış kullanıcılarla sınırlı olabilir; bu durumda yalnızca bir `anonUserId` ile gönderilen oy, `poll-login-required` hatasıyla reddedilir.

---