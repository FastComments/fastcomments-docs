[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Varsayılan olarak, FastComments oy seçeneklerini yukarı ve aşağı oklar olarak render eder; kullanıcıların bir yorumu ya yukarı ya da aşağı oylamasına olanak tanır.

Ancak, oy araç çubuğunun stilini değiştirmek mümkündür. Mevcut seçenekler varsayılan Yukarı/Aşağı düğmeleri veya Kalp tarzı bir oylama mekanizmasını kullanmaktır.

Aşağıdaki gibi **voteStyle** bayrağını kullanıyoruz:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Bunu sunucu tarafı doğrulamaları da etkinleştirdiği için kodsuz yapmanızı şiddetle öneriyoruz. Widget özelleştirme sayfasında "Vote Style" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Oylama ayrıca devre dışı bırakılabilir, stil seçeneklerinin yukarısında `Disable Voting` bölümüne bakın.