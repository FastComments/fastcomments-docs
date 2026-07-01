### Maven

Projenizin POM'una Repsy deposunu ekleyin:

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
    <!-- API İstemcisi -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Çekirdek Kütüphane (SSO içerir) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Kütüphanesi (canlı etkinlikler için) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

`build.gradle` dosyanıza Repsy deposunu ekleyin:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API İstemcisi
    implementation "com.fastcomments:client:2.0.0"
    
    // Çekirdek Kütüphane (SSO içerir)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Kütüphanesi (canlı etkinlikler için)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Bu kütüphane üç modül içerir. Oluşturulan API istemcisi, API ile çalışmayı kolaylaştıran el yazısı yardımcı işlevlerine sahip çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için kullanılan `pubsub` modülü.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Çekirdek Kütüphane Belgeleri, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Kütüphane Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerirken, `PublicApi` kimlik doğrulama olmadan doğrudan bir tarayıcı/mobil cihaz vb. üzerinden yapılabilecek yöntemleri içerir.

`ModerationApi`, kapsamlı bir canlı ve hızlı denetleme API seti sunar. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.