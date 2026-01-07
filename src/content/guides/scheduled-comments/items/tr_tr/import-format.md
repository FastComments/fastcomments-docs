### Format ornegi

Zamanlanmis Yorumlar CSV'si soyle gorunur:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Format detaylari

Zamanlanmis Yorumlar CSV dosyasi asagidaki sutunlari destekler:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

Asagidaki sutunlar **zorunludur**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Otomatik ic ice yanitlari desteklemek istiyorsaniz Parent ID sutununa ihtiyaciniz olacak.

### Format nasil calisir

Ice aktarma formati soyle calisir:

1. Yayinlamak istediginiz her yorum icin CSV'de bir satir tanimlarsiniz.
2. Yorum, belirtilen gecikmeden sonra yayinlanir (saat + dakika + saniye).
   1. Manuel olarak zamanlanan ice aktarmalar icin, gecikmeler ice aktarma tamamlandiktan sonra kullanici arayuzunde "play"e bastiginiz zamana goredir - **ice aktarma basladiginda degil**.
   2. Otomatik olarak zamanlanan ice aktarmalar icin, gecikme sayfa yukleme anindandir.
3. Bir ID tanimlamalisiniz. Basitce 1, 2, 3, 4, 5 gibi artan tanimlayicilar kullanabilirsiniz.
4. Ic ice yanitlar kullanmak istiyorsaniz, `Parent ID` sutun degerini baska bir yorumun `ID`'sine ayarlamaniz yeterlidir.
5. `Comment` alani, FastComments'in yorum widget'inda metin stillemek ve resim eklemek icin destekledigi ayni sozdizimini destekler.
6. `Avatar` alani, kullanilirsa, herkese acik erisebilir bir resim olmalidir. CDN'imize kopyalanacak ve oradan sunulacaktir.
7. Kaydirmadan sonra veya kaydirma durdurulursa tum yorumlari silebilirsiniz.
8. Silme canli olarak gerceklesir, boylece ayni zamanlanmis ice aktarmayi tekrar tekrar kullanabilirsiniz.

### Bir ornek

[Ornek bir CSV dosyasi buradadir](/csv/fastcomments-scheduled-comments-example.csv).
