[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, yerelleştirilmiş göreceli tarihler kullanılır. Örneğin, yeni bırakılmış bir yorumun yanında "11 dakika önce" görebilirsiniz.

Bu göreceli tarih formatını korumak gerekli veya istenebilir, ancak aynı zamanda tam tarihi de yanına göstermek isteyebilirsiniz; bu durumda bu parametreyi true olarak ayarlarsınız. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Bu, kod yazmadan, widget özelleştirme sayfasında Gelişmiş Seçenekler altında özelleştirilebilir. Bu seçeneği UI'da görebilmek için önce Mutlak Tarihleri etkinleştirmeniz gerekir.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Advanced Options on the widget customization page with both absolute dates and the combined relative date setting enabled'; title='Use Both Absolute and Relative Dates' app-screenshot-end]