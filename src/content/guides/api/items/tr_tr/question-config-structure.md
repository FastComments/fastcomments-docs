FastComments, soru oluşturma ve bunların sonuçlarını toplama yolu sağlar. Bir soru örneği (bundan sonra `QuestionConfig` olarak anılacaktır)
yıldız derecelendirmesi, bir kaydırıcı veya bir NPS sorusu olabilir (`type` ile belirlenir).

Soru verileri ayrı ayrı, birlikte, zaman içinde, genel olarak, sayfaya göre vb. şekilde toplanabilir.

Bu çerçeve, istemci tarafı widget'ları (bu API'nin önünde sunucunuzla), yönetici panolarını ve raporlama araçlarını oluşturmak için gereken tüm yeteneklere sahiptir.

İlk olarak bir `QuestionConfig` tanımlamamız gerekiyor. Yapısı şu şekildedir:

[inline-code-attrs-start title = 'QuestionConfig Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type QuestionConfigType = 'nps' | 'slider' | 'star' | 'thumbs';

interface QuestionConfig {
    id: string
    tenantId: string
    name: string
    question: string
    helpText?: string
    createdAt: string
    createdBy: string
    /** SADECE OKUNUR - her yeni yanıta göre artırılır. **/
    usedCount: number
    /** Yapılandırmanın en son ne zaman kullanıldığına (sonucun bırakıldığı) dair bir tarih dizgesi. **/
    lastUsed?: string
    type: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder: number
}
[inline-code-end]