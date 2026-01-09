---
Bir `HashTag` nesnesi, bir kullanıcı tarafından bırakılabilecek bir etiketi temsil eder. HashTags harici bir içeriğe bağlamak veya ilişkili yorumları birbirine bağlamak için kullanılabilir.

`HashTag` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'HashTag Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** "#" veya istenen karakterle başlamalı. **/
    tag: string
    /** Hashtag'in işaret edebileceği isteğe bağlı bir URL. Yorumları hashtag ile filtrelemek yerine, kullanıcı arayüzü tıklanınca buna yönlendirir. **/
    url?: string
    /** SADECE OKUNUR **/
    createdAt: string
}
[inline-code-end]

Notlar:

- Bazı API uç noktalarında hashtag'in URL içinde kullanıldığını göreceksiniz. Değerleri URI ile kodlamayı unutmayın. Örneğin, `#` yerine `%23` olarak temsil edilmelidir.
- Bu alanların bazıları `READONLY` olarak işaretlenmiştir - bunlar API tarafından döndürülür ancak ayarlanamaz.
 
---