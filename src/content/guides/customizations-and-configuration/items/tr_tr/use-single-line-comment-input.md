[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcının varsayılan karakter sınırına kadar istedikleri kadar satır içeren bir yorum girmesine izin verir.

Ancak, kullanıcının yalnızca tek bir satır metin girmesiyle sınırlandırılması istenebilir. Örnek kullanım durumları arasında çevrimiçi teklif verme veya canlı sohbet bulunur; FastComments
bu amaçlarla kullanılabilir.

Aşağıdaki gibi **useSingleLineCommentInput** bayrağını etkinleştiririz:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Bu, kod kullanmadan da yapılabilir. Widget özelleştirme sayfasında, "Tek Satırlık Yorum Girişini Etkinleştir" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Unutmayın ki, her sayfadaki yorumlar her sıralama yönü için önceden hesaplanır; bu nedenle tüm sıralama yönlerinin performansı aynıdır.