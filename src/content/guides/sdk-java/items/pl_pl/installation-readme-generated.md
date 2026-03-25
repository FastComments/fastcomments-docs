### Maven

Add the Repsy repository to your project's POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

```xml
<dependencies>
    <!-- Klient API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Biblioteka Core (zawiera SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Biblioteka PubSub (dla zdarzeń na żywo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Add the Repsy repository to your build.gradle file:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Klient API
    implementation "com.fastcomments:client:1.3.1"
    
    // Biblioteka Core (zawiera SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Biblioteka PubSub (dla zdarzeń na żywo)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacja biblioteki Core, w tym przykłady SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacja biblioteki PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API publiczne vs zabezpieczone

For the API client, there are two classes, `DefaultApi` and `PublicApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.