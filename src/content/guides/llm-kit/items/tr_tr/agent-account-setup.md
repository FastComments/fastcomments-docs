---
Claude Code, Cursor veya bir MCP tabanlı asistan gibi bir kodlama ajanı, kayıt formunu doldurmanıza gerek kalmadan FastComments'ı sizin için kurabilir. Bu, bir ajana "siteme yorum ekle" dediğinizde ve henüz bir hesabınız olmadığında faydalıdır.

### Nasıl çalışır

1. Ajan yeni bir hesap oluşturur ve bir API anahtarı ile bir talep bağlantısı alır. API anahtarı hemen çalışır, böylece ajan hesabı yapılandırabilir ve widget'ı sitenize kurabilir.
2. Ajan size talep bağlantısını verir. Tarayıcınızda açın, oturum açın veya bir oturum oluşturun ve talebi onaylayın. Hesap artık size ait olur: kontrol panelinden hesabı, faturalandırmasını ve API anahtarlarını yönetirsiniz. Sayfa, ajanın elinde tuttuğu API anahtarını listeler, böylece ajana ya da onu çalıştıran kişiye erişim izni vermek istemediğinizde iptal edebilirsiniz.
3. Eğer kimse 72 saat içinde talep bağlantısını açmazsa, hesap ve anahtarı silinir. Ajandan yeni bir tane oluşturmasını isteyin.

Talep edilene kadar, hesabın limitleri normal bir ücretsiz deneme gibi aynı kalır.

### Zaten bir hesabınız varsa

Her oturum bir hesabı sahiptir. Mevcut bir hesaba giriş yapmışken bir talep bağlantısı açarsanız, sayfa size seçim yapma imkanı sunar:

- **Hesabıma bağla** yeni hesabı, oturum açtığınız hesabın yönetilen bir kiracısı yapar. Bu, beyaz etiketleme içeren bir ücretli plan gerektirir ve yeni kiracının kullanımı hesabınıza faturalandırılır.
- **Çıkış yap ve başka bir oturumla talep et** sizi çıkış yapmaya zorlar ve talep sayfasına geri getirir, böylece farklı bir oturumla talep edebilirsiniz.

### Ajan yazarları için

Ajan talimatları [fastcomments.com/agents.md](https://fastcomments.com/agents.md) adresinde, hesap oluşturma çağrısını, yanıt içindeki alanları ve talep bağlantısını çalıştığınız kişiye nasıl teslim edeceğinizi açıklar. Bu çağrı bir API anahtarı gerektirmez ve IP adresine göre oran sınırlamasına sahiptir.

---