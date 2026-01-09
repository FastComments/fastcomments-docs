[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Varsayılan olarak FastComments, yanıtların sınırsız iç içe geçmesine izin verir ve kullanıcıların yanıtlara süresiz olarak yanıt verebileceği bir konu yapısı oluşturur.

maxReplyDepth seçeneği, yanıt dizilerinin ne kadar derinleşebileceğini sınırlandırmanıza olanak tanır. Maksimum derinliğe ulaşıldığında, kullanıcılar o seviyedeki yorumlarda artık bir yanıt düğmesi görmezler.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

maxReplyDepth 2 olarak ayarlandığında:
- Kullanıcılar üst düzeyde yorum yapabilir (derinlik 0)
- Kullanıcılar üst düzey yorumlara yanıt verebilir (derinlik 1)
- Kullanıcılar bu yanıtlara yanıt verebilir (derinlik 2)
- Derinlik 2'nin ötesinde daha fazla yanıt izin verilmez

1 olarak ayarlanması yalnızca üst düzey yorumlara yanıt verilmesine izin verir ve daha düz bir tartışma yapısı oluşturur.

maxReplyDepth'i 0 olarak ayarlamak tüm yanıtları devre dışı bırakır ve yalnızca üst düzey yorumlara izin verir. Belirtilmezse, yanıtlar sınırsız olarak iç içe geçebilir.