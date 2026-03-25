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
    <!-- API İstemcisi -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Çekirdek Kütüphane (SSO içerir) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Kütüphanesi (canlı etkinlikler için) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // Çekirdek Kütüphane (SSO içerir)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Kütüphanesi (canlı etkinlikler için)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

API istemcisi için iki sınıf vardır: `DefaultApi` ve `PublicApi`. `DefaultApi` API anahtarınızı gerektiren yöntemleri içerir ve `PublicApi` kimlik doğrulama olmadan doğrudan bir tarayıcı/telefon/vesaire üzerinden yapılabilecek API çağrılarını içerir.