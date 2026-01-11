Webhooks yönetiminde her olay türü (Oluşturma, Güncelleme, Silme) için `Send Test Payload` düğmeleri vardır. Oluşturma ve Güncelleme olayları sahte bir WebhookComment nesnesi gönderirken, Silme testi yalnızca bir ID içeren sahte bir istek gövdesi gönderir.

Test, "happy" (doğru API Key) ve "sad" (geçersiz API key) senaryoları için yanıt kodunu doğrulamak amacıyla iki çağrı yapacaktır.

Test geçersiz bir API key gönderdiğinde, testin tamamen geçmesi için 401 statü kodu döndürmelisiniz. token değerini doğru şekilde kontrol etmezseniz bir hata görürsünüz.

Bu, isteği doğru şekilde kimlik doğruladığınızdan emin olmak içindir.