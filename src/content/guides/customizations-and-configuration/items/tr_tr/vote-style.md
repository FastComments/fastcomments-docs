[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Varsayılan olarak, FastComments oy verme seçeneklerini yukarı ve aşağı oklar olarak gösterir ve kullanıcıların bir yorumu yukarı ya da aşağı oy vermesine izin verir.

Ancak, oy araç çubuğunun stilini değiştirmek mümkündür. Mevcut seçenekler varsayılan Yukarı/Aşağı düğmeleri veya Kalp stilinde bir oy verme mekanizmasıdır.

**voteStyle** bayrağını şu şekilde kullanıyoruz:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Kalp Düğmesini Etkinleştir'; code-example-end]

Bunu kod olmadan yapmanızı şiddetle öneririz, çünkü bu aynı zamanda sunucu tarafı doğrulamaları da etkinleştirir. Widget özelleştirme sayfasında, "Oy Stili" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Widget özelleştirme sayfasındaki Oy Verme Stili ayarı, yukarı ve aşağı okları veya kalp oylamasını sunar'; title='Oy Verme Stilini Değiştir' app-screenshot-end]

Oy verme aynı zamanda devre dışı bırakılabilir, stil seçeneklerinin üzerindeki `Disable Voting` bölümüne bakın.