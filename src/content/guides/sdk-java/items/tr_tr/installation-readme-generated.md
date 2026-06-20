### Maven

Projenizin POM dosyasına Repsy deposunu ekleyin:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Ardından ihtiyaç duyduğunuz bağımlılıkları ekleyin:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

build.gradle dosyanıza Repsy deposunu ekleyin:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Bu kütüphane üç modül içerir. Oluşturulmuş API istemcisi, API ile çalışmayı kolaylaştırmak için elle yazılmış yardımcıları içeren core Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir ve `PublicApi`, tarayıcı/mobil cihaz vb. üzerinden kimlik doğrulama olmadan doğrudan yapılabilecek yöntemleri içerir.

`ModerationApi` moderatör panosunu besler. Yorum moderasyonu için yöntemler içerir (listeleme, sayma, arama, günlükler ve dışa aktarma), moderasyon eylemleri (kaldır/geri yükle, işaretleme, inceleme/spam/onay durumunu ayarlama, oylar ve konuyu yeniden aç/kapat), yasaklar (yoruma yasaklama, yasağı geri alma, ön-yasak özetleri, yasak durumu ve tercihleri ve yasaklı kullanıcı sayıları) ve rozetler & güven (rozet verme/kaldırma, manuel rozetler, güven faktörünü al/ayarla ve kullanıcının dahili profili). Her `ModerationApi` yöntemi, çağrının SSO ile kimlik doğrulaması yapılmış bir moderatör adına gerçekleştirilebilmesi için bir `sso` parametresi kabul eder.