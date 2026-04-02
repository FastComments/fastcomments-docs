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

Sonra ihtiyacınız olan bağımlılıkları ekleyin:

```xml
<dependencies>
    <!-- API İstemcisi -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Çekirdek Kütüphane (SSO içerir) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Kütüphanesi (canlı olaylar için) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // API İstemcisi
    implementation "com.fastcomments:client:1.3.2"
    
    // Çekirdek Kütüphane (SSO içerir)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Kütüphanesi (canlı etkinlikler için)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Kütüphane İçeriği

Bu kütüphane üç modül içerir. Oluşturulmuş API istemcisi, API ile çalışmayı kolaylaştırmak için elle yazılmış yardımcılar içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abonelik için bir kütüphane olan `pubsub` modülü.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Çekirdek Kütüphane Belgeleri, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Genel vs Güvenli API'ler

API istemcisi için iki sınıf vardır: `DefaultApi` ve `PublicApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir; `PublicApi` ise kimlik doğrulama olmadan bir tarayıcı/taşınabilir cihaz vb. üzerinden doğrudan yapılabilen API çağrılarını içerir.