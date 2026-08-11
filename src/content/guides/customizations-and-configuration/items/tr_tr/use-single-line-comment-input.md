[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcının istediği kadar satır içeren bir yorum girmesine izin verir, varsayılan karakter sınırına kadar.

Ancak, kullanıcının yalnızca tek bir satır metin girmesini sınırlamak isteyebilirsiniz. Örnek kullanım durumları arasında çevrimiçi teklif verme veya FastComments'un kullanılabileceği canlı sohbet bulunur.

**useSingleLineCommentInput** bayrağını aşağıdaki gibi etkinleştiririz:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Tek Satır Yorum Girişi Etkinleştir'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Tek Satır Yorum Girişi Etkinleştir" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Widget özelleştirme sayfasında tek satır yorum girişi onay kutusu etkinleştirildi, giriş bir satırla sınırlı'; title='Tek Satır Yorum Girişi Etkinleştir' app-screenshot-end]

Not edin, her sayfadaki yorumlar her sıralama yönü için önceden hesaplanır, bu yüzden tüm sıralama yönleri aynı performansa sahiptir.