[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, yeni canlı yorumlar gerçek zamanlı olarak gönderildikçe yorum listesinin en üstünde görünür.

Bu seçenek etkinleştirildiğinde, yeni canlı yorumlar bunun yerine listenin altına eklenecektir. Bu, kullanıcılar yorum dizisini görüntülerken yorumlar gerçek zamanlı gönderildiğinde onların görünme şeklini etkiler.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Bu ayar etkinleştirildiğinde:
- Diğer kullanıcılar tarafından gönderilen yeni canlı yorumlar yorum listesinin en altında görünecektir
- Kullanıcılar, yeni yorumların mevcut yorumların altında gerçek zamanlı olarak göründüğünü görecekler
- Bu yalnızca canlı yorum güncellemelerini etkiler - sayfanın ilk yüklenmesini etkilemez
- Bu, kullanıcılar bir tartışmayı takip ederken okuma akışının korunmasına yardımcı olabilir

Bu ayarın yalnızca yeni canlı yorumlar gerçek zamanlı olarak ulaştıklarında nereye yerleştirildiklerini etkilediğini unutmayın. Sayfa yüklendiğinde uygulanan ilk sıralamayı etkilemez.