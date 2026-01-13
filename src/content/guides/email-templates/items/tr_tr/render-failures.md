E-posta şablonları değişkenleri ve mantığı desteklediği için, render edilemeyen veya bazen render olmayan şablonlar oluşturmak mümkündür.

Bu, özellikle ara sıra ortaya çıkan bir sorun ise veya yalnızca veriler belirli bir şekilde olduğunda meydana geliyorsa, teşhis etmek ve hata ayıklamak açısından çok sinir bozucu olabilir.

Yardımcı olmak için, FastComments E-posta Şablonları birkaç özellikle gelir:

1. Şablon önizlemesi başarısız olursa, kaydedilemez. Bir hata mesajı gösterilir.
2. Şablon render hataları yönetici arayüzünde izlenir ve raporlanır.

İkinci madde, üretimde meydana gelen render hatalarını tanımlar. Yani, önizlemesi düzgün olan bir şablon oluşturursunuz - ancak daha sonra bir sebepten dolayı başarısız olur. Örneğin, şablonumuzda şunun gibi bir şey varsa:

    <% if (comment.commenterEmail.includes('test') { %>

Bu, anonim yorum yapma etkinleştirilmişse bazen başarısız olabilir, çünkü e-posta her zaman mevcut olmayacaktır. Peki bunu nasıl öğreniriz?

Cevap, hataların iki yerde ortaya çıkarılmasıdır. İlk olarak, şablon listesinin kendisi her şablonla birlikte bir render hata sayısı gösterir.

Daha sonra, bir şablonu görüntülerken, şablonun render olamama sayısının her hata için bir sayısını görebiliriz.

Her hatanın ve onun sayısının yanında bir sıfırlama düğmesi bulunur, böylece bir sorunu çözdükten sonra sayacı sıfırlayabiliriz.