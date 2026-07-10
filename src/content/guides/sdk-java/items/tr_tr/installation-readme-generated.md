### Maven

Repsy deposunu projenizin POM'una ekleyin:

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
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Repsy deposunu `build.gradle` dosyanıza ekleyin:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Library Contents

Bu kütüphane üç modül içerir. Oluşturulan API istemcisi, API ile çalışmayı kolaylaştıran el yazısı yardımcı programları içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerirken, `PublicApi` kimlik doğrulama olmadan bir tarayıcı/mobil cihaz vb. üzerinden doğrudan yapılabilen yöntemleri içerir.

`ModerationApi`, canlı ve hızlı denetleme API'lerinin kapsamlı bir paketini sunar. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.