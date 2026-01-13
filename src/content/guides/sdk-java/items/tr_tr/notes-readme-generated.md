### Broadcast Kimlikleri

Bazı API çağrılarında bir `broadcastId` geçirmeniz gerektiğini göreceksiniz. Olayları aldığınızda bu ID size geri dönecek, böylece istemci tarafında değişiklikleri iyimserce uygulamayı planlıyorsanız olayı görmezden geleceğinizi bilirsiniz
(muhtemelen en iyi deneyimi sağladığı için bunu yapmak isteyeceksiniz). Burada bir UUID gönderin. ID, bir tarayıcı oturumunda iki kez tekrar etmeyecek kadar benzersiz olmalıdır.