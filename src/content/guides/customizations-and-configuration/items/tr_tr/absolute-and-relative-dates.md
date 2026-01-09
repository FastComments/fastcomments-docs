---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, yerelleştirilmiş göreli tarihler kullanılır. Örneğin, yakın zamanda bırakılan bir yorumun yanında "11 dakika önce" görebilirsiniz.

Bu göreli tarih biçimini korumak gerekebilir veya isteğe bağlı olabilir; ancak aynı zamanda tam tarihi de yanında göstermek istiyorsanız, bu parametreyi true olarak ayarlarsınız. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Bu, kod yazmadan widget özelleştirme sayfasında, Gelişmiş Seçenekler altında özelleştirilebilir. Bu seçeneği kullanıcı arayüzünde (UI) görebilmek için önce Mutlak Tarihleri etkinleştirmeniz gerekir.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---