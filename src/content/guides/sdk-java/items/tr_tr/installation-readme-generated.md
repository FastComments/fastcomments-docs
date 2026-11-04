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
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (SSO içerir) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (canlı etkinlikler için) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (SSO içerir)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (canlı etkinlikler için)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Kütüphane İçeriği

Bu kütüphane üç modül içerir. Üretilmiş API istemcisi, API ile çalışmayı kolaylaştıran elle yazılmış yardımcı araçlar içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API Client Library Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [SSO Örnekleri Dahil Core Library Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Dokümantasyonu](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Herkese Açık ve Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren metotları içerir; `PublicApi` ise kimlik doğrulama olmadan doğrudan bir tarayıcıdan/mobil cihazdan vb. yapılabilecek metotları içerir.

`ModerationApi`, canlı ve hızlı moderasyon için kapsamlı bir API paketi sunar. Her `ModerationApi` metodu bir `sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.