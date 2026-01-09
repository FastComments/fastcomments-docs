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

Daha sonra ihtiyacınız olan bağımlılıkları ekleyin:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
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
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Library Contents

Bu kütüphane üç modül içerir. Oluşturulmuş API istemcisi, API ile çalışmayı kolaylaştırmak için elle yazılmış yardımcılar içeren çekirdek Java kütüphanesi ve değişiklik akışlarına abone olmak için bir kütüphane olan `pubsub` modülü.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Çekirdek Kütüphane Belgeleri (SSO Örnekleri Dahil)](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API istemcisi için iki sınıf vardır, `DefaultApi` ve `PublicApi`. `DefaultApi` API anahtarınızı gerektiren yöntemleri içerir, ve `PublicApi` kimlik doğrulama olmadan bir tarayıcı/cep cihazı vb. üzerinden doğrudan yapılabilecek API çağrılarını içerir.