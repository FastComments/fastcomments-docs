### Yayın Kimlikleri

Bazı API çağrılarında bir `broadcastId` geçirmeniz gerektiğini göreceksiniz. Olayları aldığınızda bu ID'yi geri alırsınız, bu sayede istemcide değişiklikleri iyimserce uygulamayı planlıyorsanız olayı yok saymanız gerektiğini bilirsiniz
(bunu muhtemelen yapmak isteyeceksiniz çünkü en iyi deneyimi sunar). Burada bir UUID gönderin. ID, bir tarayıcı oturumu içinde iki kez oluşmayacak kadar benzersiz olmalıdır.