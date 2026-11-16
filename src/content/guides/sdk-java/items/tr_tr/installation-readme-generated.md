### Maven

Repsy deposunu projenizin POM dosyasına ekleyin:

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
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Repsy deposunu build.gradle dosyanıza ekleyin:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Kütüphane İçeriği

Bu kütüphane üç modül içerir. Üretilmiş API istemcisi, API ile çalışmayı kolaylaştıran elle yazılmış yardımcı araçlar içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API İstemci Kütüphanesi Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Çekirdek Kütüphane Dokümantasyonu, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Kütüphanesi Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Herkese Açık ve Güvenli API'ler

API istemcisinde üç sınıf bulunur: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren metotları içerir; `PublicApi` ise kimlik doğrulama gerektirmeden doğrudan bir tarayıcıdan/mobil cihazdan/vb. yapılabilecek metotları içerir.

`ModerationApi`, canlı ve hızlı moderasyon için kapsamlı bir API seti sunar. Her `ModerationApi` metodu bir `sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.