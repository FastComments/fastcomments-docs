Varsayılan olarak, FastComments kullanıcının adını girdiği şekilde veya SSO aracılığıyla bize iletildiği şekilde gösterir.

Ancak, kullanıcının adını maskelemek veya farklı şekilde göstermek isteyebilirsiniz. Örneğin, kullanıcının adı Allen Rex ise, belki
yalnızca "Allen R." göstermek istersiniz.

Bu, kod yazmadan Widget Özelleştirme UI'sında `Commenter Name Format` adlı ayar altında yapılabilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

Mevcut biçimler şunlardır:

- Capitalize (örnek kullanıcıyı Example User olarak gösterir)
- Last Initial (Example User'ı Example U. olarak gösterir)
- All Initials (Example User'ı E. U. olarak gösterir)
- Show "Anonymous"

Bu değişikliğin etkisi hemen geçerlidir. Kullanıcılar, kendileri için yorum alanının üst kısmında yine tam kullanıcı adlarını görecekler, ancak yorumlarında
değiştirilmiş kullanıcı adı gösterilecektir.

Kullanıcı adları, kullanıcıları korumak için sunucu tarafında maskelenir.