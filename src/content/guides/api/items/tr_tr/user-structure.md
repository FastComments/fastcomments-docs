`User` tüm kullanıcı türlerinin en yaygın ortak paydasını temsil eden bir nesnedir.

Unutmayın ki FastComments'ta kullanıcılar için birçok farklı kullanım durumu bulunur:

- Secure SSO
- Simple SSO
- Tenant Users (Örneğin: Yöneticiler)
- Commenters

Bu API **Commenters** ve **Simple SSO** aracılığıyla oluşturulan kullanıcılar içindir. Temelde, siteniz üzerinden oluşturulan herhangi bir kullanıcıya bu API aracılığıyla erişilebilir. Tenant Users da bu şekilde alınabilir, ancak `/tenant-users/` API'si ile etkileşime girerek daha fazla bilgi edinebilirsiniz.

`Secure SSO` için lütfen `/sso-users/` API'sini kullanın.

Bu tür kullanıcıları güncelleyemezsiniz. Hesaplarını siteniz aracılığıyla oluşturmuşlardır, bu yüzden temel bazı salt okunur erişim sağlıyoruz, ancak değişiklik yapamazsınız. Bu tür bir akışa sahip olmak istiyorsanız - `Secure SSO`'yu kurmanız gerekir.

`User` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Kullanıcı Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Bu, yorum nesnelerindeki userId olarak da kullanılan id'dir. **/
    id: string
    username: string
    /** Örneğin yorumcunun bloguna bir bağlantı. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]