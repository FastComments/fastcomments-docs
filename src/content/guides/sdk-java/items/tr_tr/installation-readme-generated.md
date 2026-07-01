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

Ardından ihtiyacınız olan bağımlılıkları ekleyin:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Repsy deposunu **build.gradle** dosyanıza ekleyin:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Kütüphane İçerikleri

Bu kütüphane üç modül içerir. Oluşturulan API istemcisi, API ile çalışmayı kolaylaştıran el yazısı yardımcı programlar içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API İstemci Kütüphanesi Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Çekirdek Kütüphane Dokümantasyonu, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Kütüphane Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Genel vs Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerirken, `PublicApi` kimlik doğrulama gerektirmeden bir tarayıcı/mobil cihaz vb. üzerinden doğrudan yapılabilen yöntemleri içerir.

`ModerationApi`, canlı ve hızlı denetleme API'lerinin kapsamlı bir setini sağlar. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi ile kimlik doğrulaması yapabilir.