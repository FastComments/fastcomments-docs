Birden fazla kullanıcının nasıl etkileşime girdiğini tanımlamak ve bunu test etmek karmaşıktır. İşte erişim kontrolü için izlediğimiz aşağıdaki spesifikasyon; uygulamanızı test etmek için kullanabilirsiniz:

    Sayfa null group ids ile, kullanıcı null group ids ile - erişim hakkı olmalı.
    Sayfa null group ids ile, kullanıcı group ids ile - erişim hakkı olmalı.
    Sayfa group ids ile, kullanıcı null group ids ile - erişim hakkı olmalı.
    Sayfa group ids ile, kullanıcı boş liste ile - erişim hakkı OLMAMALI.
    Sayfa group ids ile, kullanıcı group ids ile - kesişim mevcut - erişim hakkı olmalı.
    Sayfa group ids ile, kullanıcı group ids ile - kesişim yok - erişim hakkı OLMAMALI.
    Sayfa group ids'in boş listesi (kimsenin erişimi yok), kullanıcı null - erişim hakkı OLMAMALI.
    
    SSO User A = Group ids tanımlı değil (null = tam erişim).
    SSO User B = Group ids tanımlı değil (null = tam erişim).
    A, B'yi @ ile etiketleyebilir.
    
    SSO User A = Group ids tanımlı değil (null = tam erişim).
    SSO User B = Group ids tanımlı.
    A, B'yi @ ile etiketleyebilir.
    
    SSO User A = Group ids tanımlı.
    SSO User B = Group ids tanımlı değil (null = tam erişim).
    A, B'yi @ ile etiketleyebilir.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A, B'yi @ ile etiketleyemez.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A, B'yi @ ile etiketleyebilir.