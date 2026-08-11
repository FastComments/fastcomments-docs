[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, üst düzey yorumların yanıtları gösterilir.

Bu, kullanıcının üst düzey yorumlarda yanıtları görmek için "Show Replies" tıklaması gerekir.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Üst Düzey Yorumlarda Yanıtları Daralt'; code-example-end]

Bu, kod yazmadan, widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Widget özelleştirme arayüzündeki yanıtları daraltma seçeneği, alt yorumları bir Show Replies bağlantısının arkasına gizler'; title='Yanıtları Daralt' app-screenshot-end]

Bu ayar, başlangıçta yüklenen üst düzey yorum sayısını etkilemez. Eğer bir üst düzey yorumunuz ve 29 alt yorumu varsa, bu ayar açıkken şunları görürsünüz:

- Üst düzey yorumu gör.
- Bu yorumun altında Show Replies (29) gör.

Bu seçenekle birlikte tüm üst düzey yorumları göstermek isterseniz, [starting page to -1](#starting-page) ayarını yapın.