#### Canvas'ta Developer Keys'i Açın

Canvas'a yönetici olarak giriş yapın. Sol kenar çubuğunda **Admin**'e gidin > hesabınızı seçin > **Developer Keys**.

#### Bir LTI Developer Key Oluşturun

**+ Developer Key**'e tıklayın ve **LTI Key**'i seçin.

Yapılandırma formunda:

1. Sol taraftaki **Redirect URIs** alanına FastComments kurulum sayfasından **Launch URL**'yi yapıştırın.
2. Sağ tarafta **Method**'u **Enter URL** olarak ayarlayın.
3. FastComments'tan kopyaladığınız **Configuration URL**'yi **JSON URL** alanına yapıştırın.
4. Canvas LTI yapılandırmasını otomatik olarak yükleyecektir.
5. Anahtara bir isim verin (ör. "FastComments").
6. **Save**'e tıklayın.

#### Developer Key'i Etkinleştirin

Kaydettikten sonra yeni anahtar Developer Keys tablosunda **State**'i **OFF** olarak görünecektir. Ayarı **ON** yapmak için açma/kapama düğmesine tıklayın. Canvas onay isteyebilir — anahtarı etkinleştirmek için **Allow**'a tıklayın.

#### Client ID'yi Kopyalayın

Developer Keys tablosu Detaylar sütununda sayısal bir **Client ID** gösterir (ör. `17000000000042`). Bu numarayı kopyalayın - sonraki adımda ihtiyacınız olacak.