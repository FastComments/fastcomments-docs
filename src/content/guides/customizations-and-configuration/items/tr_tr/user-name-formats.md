Varsayılan olarak, FastComments kullanıcının adını, kullanıcının girdiği şekilde veya SSO aracılığıyla bize iletildiği şekilde gösterir.

Ancak, kullanıcının adını maskelemek veya farklı bir şekilde göstermek istenebilir. Örneğin, kullanıcının adı Allen Rex ise, belki de yalnızca "Allen R." göstermek istersiniz.

Bu, Widget Özelleştirme arayüzünde kod yazmadan, `Commenter Name Format` adlı ayar altından yapılabilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format açılır menüsü, Capitalize, Last Initial ve All Initials gibi seçeneklerle açık halde'; title='Ad Formatını Değiştir' app-screenshot-end]

Kullanılabilir formatlar şunlardır:

- Capitalize (örnek kullanıcıyı Example User olarak gösterir)
- Last Initial (Example User kullanıcısını Example U. olarak gösterir)
- All Initials (Example User kullanıcısını E. U. olarak gösterir)
- "Anonymous" göster

Bu değişikliğin etkisi anlıktır. Kullanıcılar, yorum alanının üstünde kendi tam kullanıcı adlarını görmeye devam eder, ancak yorumları değiştirilmiş kullanıcı adını gösterir.

Kullanıcı adları, kullanıcıları korumak için sunucu tarafında maskelenir.